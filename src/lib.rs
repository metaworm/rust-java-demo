use jni::objects::*;
use jni::sys::{jint, jobject};
use jni::JNIEnv;

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_init(env: JNIEnv, _class: JClass) {
    println!("rust-java-demo inited");
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_addInt(
    env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_divInt(
    env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    if b == 0 {
        env.throw_new("Ljava/lang/Exception;", "divide zero")
            .expect("throw");
        0
    } else {
        a / b
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_getThisField(
    env: JNIEnv,
    this: JObject,
    name: JString,
    sig: JString,
) -> jobject {
    let result = env
        .get_field(
            this,
            &env.get_string(name).unwrap().to_string_lossy(),
            &env.get_string(sig).unwrap().to_string_lossy(),
        )
        .unwrap();
    result.l().unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_getThisFieldSafely(
    env: JNIEnv,
    this: JObject,
    name: JString,
    sig: JString,
) -> jobject {
    let result = (|| {
        env.get_field(
            this,
            &env.get_string(name)?.to_string_lossy(),
            &env.get_string(sig)?.to_string_lossy(),
        )?
        .l()
    })();
    match result {
        Ok(res) => res.into_inner(),
        Err(err) => {
            env.exception_clear().expect("clear");
            env.throw_new("Ljava/lang/Exception;", format!("{err:?}"))
                .expect("throw");
            std::ptr::null_mut()
        }
    }
}

#[allow(non_snake_case)]
fn call_java(env: &JNIEnv) {
    match (|| {
        let File = env.find_class("java/io/File")?;
        // 获取静态字段
        let separator = env.get_static_field(File, "separator", "Ljava/lang/String;")?;
        let separator = env
            .get_string(separator.l()?.into())?
            .to_string_lossy()
            .to_string();
        println!("File.separator: {}", separator);
        assert_eq!(separator, format!("{}", std::path::MAIN_SEPARATOR));
        // env.get_static_field_unchecked(class, field, ty)

        // 创建实例对象
        let file = env.new_object(
            "java/io/File",
            "(Ljava/lang/String;)V",
            &[JValue::Object(env.new_string("")?.into())],
        )?;

        // 调用实例方法
        let abs = env.call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])?;
        let abs_path = env
            .get_string(abs.l()?.into())?
            .to_string_lossy()
            .to_string();
        println!("abs_path: {}", abs_path);

        jni::errors::Result::Ok(())
    })() {
        Ok(_) => {}
        // 捕获异常
        Err(jni::errors::Error::JavaException) => {
            let except = env.exception_occurred().expect("exception_occurred");
            let err = env
                .call_method(except, "toString", "()Ljava/lang/String;", &[])
                .and_then(|e| Ok(env.get_string(e.l()?.into())?.to_string_lossy().to_string()))
                .unwrap_or_default();
            env.exception_clear().expect("clear exception");
            println!("call java exception occurred: {err}");
        }
        Err(err) => {
            println!("call java error: {err:?}");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_callJava(env: JNIEnv) {
    println!("call java");
    call_java(&env)
}

#[no_mangle]
pub unsafe extern "C" fn Java_pers_metaworm_RustJNI_callJavaThread(env: JNIEnv) {
    let vm = env.get_java_vm().expect("get jvm");
    std::thread::spawn(move || {
        println!("call java in another thread");
        let env = vm.attach_current_thread().expect("attach");
        call_java(&env);
    })
    .join()
    .unwrap();
}
