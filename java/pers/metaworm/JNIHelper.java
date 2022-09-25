package pers.metaworm;

import java.io.File;
import java.lang.reflect.Field;

public class JNIHelper {
    static void addLibraryPath(String path) throws NoSuchFieldException, SecurityException, IllegalArgumentException, IllegalAccessException {
        File dir = new File(path);
        System.setProperty("java.library.path", 
            System.getProperty("java.library.path") + ":" + dir.getAbsolutePath()
        );
        // System.out.println(System.getProperty("java.library.path"));
        Field field = ClassLoader.class.getDeclaredField("sys_paths");
        field.setAccessible(true);
        field.set(ClassLoader.getSystemClassLoader(), new String[]{dir.getAbsolutePath()});
    }
}
