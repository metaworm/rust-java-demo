#[allow(non_snake_case)]
pub mod cache {
    use anyhow::Context;
    use jni::errors::Result as JniResult;
    use jni::objects::*;
    use jni::JNIEnv;

    pub fn method_global_ref<'a>(
        env: JNIEnv<'a>,
        class: JClass,
        name: &str,
        sig: &str,
    ) -> JniResult<JMethodID<'a>> {
        let method = env.get_method_id(class, name, sig)?.into_inner();
        Ok(JMethodID::from(method.cast()))
    }

    pub fn static_method_global_ref<'a>(
        env: JNIEnv<'a>,
        class: JClass,
        name: &str,
        sig: &str,
    ) -> ::jni::errors::Result<JStaticMethodID<'a>> {
        let method = env.get_static_method_id(class, name, sig)?.into_inner();
        Ok(JStaticMethodID::from(method.cast()))
    }

    macro_rules! gen_global_ref {
        (@method_type) => { JMethodID<'static> };
        (@method_type static) => { JStaticMethodID<'static> };

        (@method_ref) => { method_global_ref };
        (@method_ref static) => { static_method_global_ref };

        (
            $(
                #[name = $classname:literal]
                class $name:ident {
                    $($method:ident : $($modify:ident)* $sig:literal,)*
                }
            )*
        ) => {
            $(
                #[allow(non_snake_case)]
                pub struct $name {
                    pub class: JClass<'static>,
                    $(pub $method: gen_global_ref!(@method_type $($modify)*),)*
                }

                impl $name {
                    pub fn from_env(env: JNIEnv<'static>) -> anyhow::Result<Self> {
                        Self::from_class(env, env.find_class($classname)?)
                    }

                    pub fn from_class(env: JNIEnv<'static>, class: JClass) -> anyhow::Result<Self> {
                        let cls = env.new_global_ref(class)?;
                        let class = JClass::from(*cls.as_obj());
                        core::mem::forget(cls);
                        Ok(Self {
                            class,
                            $(
                                $method: gen_global_ref!(@method_ref $($modify)*)(
                                    env, class, stringify!($method), $sig).context(stringify!($method)
                                )?,
                            )*
                        })
                    }
                }

                // TODO: impl Drop
            )*

            pub struct CachedClasses {
                $(pub $name: $name,)*
            }

            impl CachedClasses {
                pub fn from_env(env: JNIEnv<'static>) -> anyhow::Result<Self> {
                    Ok(Self {
                        $($name: $name::from_env(env).context(stringify!($name))?,)*
                    })
                }
            }

            unsafe impl Sync for CachedClasses {}
            unsafe impl Send for CachedClasses {}
        }
    }

    gen_global_ref! {
        #[name = "java/lang/Thread"]
        class Thread {
            currentThread: static "()Ljava/lang/Thread;",
            getStackTrace: "()[Ljava/lang/StackTraceElement;",
        }

        #[name = "java/lang/StackTraceElement"]
        class StackTraceElement {
            getLineNumber: "()I",
            toString: "()Ljava/lang/String;",
        }

        #[name = "java/io/File"]
        class File {
            getAbsolutePath: "()Ljava/lang/String;",
        }
    }

    static mut CLASSES: Option<Box<CachedClasses>> = None;

    pub unsafe fn init(env: JNIEnv<'static>) -> anyhow::Result<Option<Box<CachedClasses>>> {
        Ok(CLASSES.replace(CachedClasses::from_env(env)?.into()))
    }

    pub fn get() -> &'static CachedClasses {
        unsafe { CLASSES.as_ref().expect("Cached Java Classed not inited") }
    }
}
