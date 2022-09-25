package pers.metaworm;

public class RustJNI {
    static {
        System.loadLibrary("rust_java_demo");
    }

    public static void main(String[] args) {
        init();

        System.out.println("test addInt: " + (addInt(1, 2) == 3));

        RustJNI jni = new RustJNI();
        System.out.println("test getThisField: " + (jni.getThisField("stringField", "Ljava/lang/String;") == jni.stringField));

        try {
            System.out.println("test getThisFieldSafely: " + (jni.getThisFieldSafely("stringField", "Ljava/lang/String;") == jni.stringField));
            jni.getThisFieldSafely("fieldNotExists", "Ljava/lang/String;");
        } catch (Exception e) {
            System.out.println("test getThisFieldSafely: catched exception: " + e.toString());
        }

        try {
            System.out.println("test divInt: " + divInt(3, 0));
        } catch (Exception e) {
            System.out.println("test divInt: catched exception: " + e.toString());
        }

        callJava();
        callJavaThread();

        System.out.println("test success");
    }

    String stringField = "abc";

    static native void init();
    static native int addInt(int a, int b);
    static native int divInt(int a, int b);
    static native int callJava();
    static native int callJavaThread();

    native Object getThisField(String name, String sig);
    native Object getThisFieldSafely(String name, String sig);
}
