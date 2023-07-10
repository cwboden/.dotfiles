import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.openapitools.generator.gradle.plugin.tasks.GenerateTask

plugins {
    id("org.springframework.boot") version "3.1.1"
    id("io.spring.dependency-management") version "1.1.0"
    id("org.openapi.generator") version "6.6.0"
    kotlin("jvm") version "1.8.22"
    kotlin("plugin.spring") version "1.8.22"
}

group = "com.cwboden"
version = "0.0.1-SNAPSHOT"

java {
    sourceCompatibility = JavaVersion.VERSION_17
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.springframework.boot:spring-boot-starter")
    implementation("org.springframework.boot:spring-boot-starter-web")
    implementation("org.springframework.boot:spring-boot-starter-actuator")

    implementation("org.jetbrains.kotlin:kotlin-reflect")
    implementation("jakarta.validation:jakarta.validation-api")
    implementation("io.swagger.core.v3:swagger-annotations:2.2.14")
    implementation("io.swagger.core.v3:swagger-models:2.2.14")

    implementation("com.fasterxml.jackson.core:jackson-core")
    implementation("com.fasterxml.jackson.core:jackson-annotations")

    testImplementation("org.springframework.boot:spring-boot-starter-test")
}

tasks.withType<KotlinCompile> {
    kotlinOptions {
        freeCompilerArgs += "-Xjsr305=strict"
        jvmTarget = "17"
    }
}

tasks.withType<Test> {
    useJUnitPlatform()
}

tasks.register<GenerateTask>("generateServerStubs") {
    generatorName.set("kotlin-spring")
    inputSpec.set("$rootDir/openapi-specs/petstore.yaml")
    outputDir.set("$buildDir/generated")
    apiPackage.set("com.cwboden.generated.api")
    invokerPackage.set("com.cwboden.generated.invoker")
    modelPackage.set("com.cwboden.generated.model")

    additionalProperties.set(mapOf(
            "library" to "spring-boot",
            "beanValidations" to true,
            "swaggerAnnotations" to true,
            "serviceInterface" to true,
            "useSpringBoot3" to true,
    ))
}

configure<SourceSetContainer> {
    named("main") {
        kotlin.srcDir("$buildDir/generated/src/main/kotlin")
    }
}