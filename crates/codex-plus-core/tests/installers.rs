use codex_plus_core::install::{
    InstallOptions, SILENT_BINARY, app_bundle_names, build_macos_app_bundle,
    build_windows_entrypoint_plan, companion_binary_path_from_exe, default_install_root_strategy,
    shortcut_names,
};

#[test]
fn windows_entrypoint_plan_contains_silent_and_manager_entrypoints() {
    let options = InstallOptions {
        install_root: Some("C:/Users/A/Desktop".into()),
        launcher_path: Some("C:/Tools/codexforge.exe".into()),
        manager_path: Some("C:/Tools/codexforge-manager.exe".into()),
        remove_owned_data: false,
    };

    let plan = build_windows_entrypoint_plan(&options);

    assert!(plan.silent_shortcut.ends_with("CodexForge.lnk"));
    assert!(plan.manager_shortcut.ends_with("CodexForge 管理工具.lnk"));
    assert_eq!(plan.launcher_path, "C:/Tools/codexforge.exe");
    assert_eq!(plan.manager_path, "C:/Tools/codexforge-manager.exe");
    assert_eq!(plan.silent_icon_path, "C:/Tools/codexforge.exe");
    assert_eq!(plan.manager_icon_path, "C:/Tools/codexforge-manager.exe");
    assert_eq!(plan.uninstall_key, "CodexForge");
    assert_eq!(plan.legacy_uninstall_key, "CodexForge");
    assert_eq!(
        plan.uninstaller_path.replace('\\', "/"),
        "C:/Tools/uninstall.exe"
    );
    assert_eq!(
        plan.uninstall_command.replace('\\', "/"),
        "\"C:/Tools/uninstall.exe\""
    );
    assert_eq!(
        plan.quiet_uninstall_command.replace('\\', "/"),
        "\"C:/Tools/uninstall.exe\" /S"
    );
    assert_ne!(
        plan.uninstall_command,
        "\"C:/Tools/codexforge-manager.exe\""
    );
}

#[test]
fn windows_entrypoint_plan_can_request_owned_data_removal_without_shell_script() {
    let options = InstallOptions {
        install_root: Some("C:/Users/A/Desktop".into()),
        launcher_path: None,
        manager_path: None,
        remove_owned_data: true,
    };

    let plan = build_windows_entrypoint_plan(&options);

    assert!(plan.silent_shortcut.ends_with("CodexForge.lnk"));
    assert!(plan.manager_shortcut.ends_with("CodexForge 管理工具.lnk"));
    assert!(plan.remove_owned_data);
}

#[test]
fn macos_bundle_metadata_contains_silent_and_manager_apps() {
    let options = InstallOptions {
        install_root: Some("/Applications".into()),
        launcher_path: Some("/opt/CodexForge/codexforge".into()),
        manager_path: Some("/opt/CodexForge/codexforge-manager".into()),
        remove_owned_data: false,
    };

    let silent = build_macos_app_bundle(&options, false);
    let manager = build_macos_app_bundle(&options, true);

    assert!(silent.app_path.ends_with("CodexForge.app"));
    assert!(manager.app_path.ends_with("CodexForge 管理工具.app"));
    assert!(silent.info_plist.contains("<string>CodexForge</string>"));
    assert!(
        manager
            .info_plist
            .contains("<string>CodexForge 管理工具</string>")
    );
    assert_eq!(silent.binary_target_name.as_deref(), Some("codexforge"));
    assert_eq!(
        manager.binary_target_name.as_deref(),
        Some("codexforge-manager")
    );
    assert!(silent.launch_script.contains("$DIR/codexforge"));
    assert!(manager.launch_script.contains("$DIR/codexforge-manager"));
}

#[test]
fn installer_exports_expected_two_entrypoint_names() {
    assert_eq!(
        shortcut_names(),
        ("CodexForge.lnk", "CodexForge 管理工具.lnk")
    );
    assert_eq!(
        app_bundle_names(),
        ("CodexForge.app", "CodexForge 管理工具.app")
    );
}

#[test]
fn macos_dmg_includes_applications_shortcut_for_drag_install() {
    let script = std::fs::read_to_string("../../scripts/installer/macos/package-dmg.sh")
        .expect("read macOS DMG packaging script");

    assert!(script.contains("ln -s /Applications \"$STAGE/Applications\""));
}

#[test]
fn companion_binary_path_resolves_macos_silent_app_next_to_manager_app() {
    let manager_exe = std::path::Path::new(
        "/Applications/CodexForge 管理工具.app/Contents/MacOS/CodexForgeManager",
    );

    let companion = companion_binary_path_from_exe(manager_exe, SILENT_BINARY);

    assert_eq!(
        companion,
        std::path::PathBuf::from("/Applications/CodexForge.app/Contents/MacOS/CodexForge")
    );
    assert_ne!(
        companion,
        std::path::PathBuf::from("/Applications/CodexForge 管理工具.app/Contents/MacOS/codexforge")
    );
}

#[test]
fn companion_binary_path_resolves_macos_manager_app_next_to_silent_app() {
    let silent_exe = std::path::Path::new("/Applications/CodexForge.app/Contents/MacOS/CodexForge");

    let companion =
        companion_binary_path_from_exe(silent_exe, codex_plus_core::install::MANAGER_BINARY);

    assert_eq!(
        companion,
        std::path::PathBuf::from(
            "/Applications/CodexForge 管理工具.app/Contents/MacOS/CodexForgeManager"
        )
    );
}

#[test]
fn macos_bundle_does_not_wrap_the_bundle_executable_in_itself() {
    let options = InstallOptions {
        install_root: Some("/Applications".into()),
        launcher_path: Some("/Applications/CodexForge.app/Contents/MacOS/CodexForge".into()),
        manager_path: Some(
            "/Applications/CodexForge 管理工具.app/Contents/MacOS/CodexForgeManager".into(),
        ),
        remove_owned_data: false,
    };

    let silent = build_macos_app_bundle(&options, false);
    let manager = build_macos_app_bundle(&options, true);

    assert_eq!(
        silent.binary_source,
        Some(std::path::PathBuf::from(
            "/Applications/CodexForge.app/Contents/MacOS/CodexForge"
        ))
    );
    assert_eq!(
        manager.binary_source,
        Some(std::path::PathBuf::from(
            "/Applications/CodexForge 管理工具.app/Contents/MacOS/CodexForgeManager"
        ))
    );
    assert!(silent.launch_script.contains("$DIR/codexforge"));
    assert!(manager.launch_script.contains("$DIR/codexforge-manager"));
}

#[test]
fn windows_default_install_root_uses_known_folder_before_userprofile_desktop() {
    let strategy = default_install_root_strategy();

    if cfg!(windows) {
        assert_eq!(strategy, "windows-known-folder");
    } else if cfg!(target_os = "macos") {
        assert_eq!(strategy, "macos-applications");
    } else {
        assert_eq!(strategy, "user-dirs-desktop");
    }
}
