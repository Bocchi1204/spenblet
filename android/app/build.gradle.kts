plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "org.spenblet.app"
    compileSdk = 35

    defaultConfig {
        applicationId = "org.spenblet.app"
        minSdk = 26
        targetSdk = 35
        versionCode = 100
        versionName = "1.0.0-beta.1"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}
