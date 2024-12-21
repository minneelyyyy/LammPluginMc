package com.minneelyyyy.plugin;

import org.bukkit.plugin.java.JavaPlugin;

import java.util.Objects;

public class LammPluginMc extends JavaPlugin {
    @Override
    public void onEnable() {
        Objects.requireNonNull(this.getCommand("lamm")).setExecutor(new CommandLamm());
    }
}
