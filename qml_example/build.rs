use semver::Version;

fn main() {
    let qt_include_path = std::env::var("DEP_QT_INCLUDE_PATH").unwrap();
    let qt_library_path = std::env::var("DEP_QT_LIBRARY_PATH").unwrap();
    let qt_version = std::env::var("DEP_QT_VERSION")
        .unwrap()
        .parse::<Version>()
        .expect("Parsing Qt version failed");

    if qt_version >= Version::new(6, 0, 0) {
        // This example is not supported on Qt 6 and above because graphics
        // API used for it were removed.
        println!("cargo:rustc-cfg=no_qt");
        return;
    }

    #[allow(unused_mut)]
    let mut config = cpp_build::Config::new();

    if cfg!(target_os = "macos") {
        config.flag("-F");
        config.flag(&qt_library_path);
    }

    config
        .include(&qt_include_path)
        .include(format!("{}/QtQuick", qt_include_path))
        .include(format!("{}/QtCore", qt_include_path))
        .include(format!("{}/QtGui", qt_include_path))
        .include(format!("{}/QtWebEngine", qt_include_path))
        .include(format!("{}/QtWidgets", qt_include_path))
        // See https://github.com/woboq/qmetaobject-rs/pull/168
        //
        // QSGSimpleMaterial{,Shader} classes ain't going to be removed from Qt5
        // which is on a life support at this point; and we know for sure they are
        // already gone in Qt6. So, there's just no point seeing these warning
        // over and over again.
        .flag_if_supported("-Wno-deprecated-declarations")
        .flag_if_supported("-std=c++17")
        .build("src/lib.rs");
}
