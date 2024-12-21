package com.minneelyyyy.plugin;

import io.questdb.jar.jni.JarJniLoader;

public class Lamm {
    public static native String runCode(String code);

    static {
        JarJniLoader.loadLib(
                Lamm.class,

                // A platform-specific path is automatically suffixed to path below.
                "/com/minneelyyyy/plugin/libs",

                // The "lib" prefix and ".so|.dynlib|.dll" suffix are added automatically as needed.
                "lamm_plugin_mc_backend");
    }
}
