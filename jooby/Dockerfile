FROM gradle:7.4-jdk11 as build
WORKDIR /jooby
COPY build.gradle build.gradle
COPY settings.gradle settings.gradle
COPY src src
COPY conf conf
RUN gradle shadowJar

FROM openjdk:11-jdk-slim
WORKDIR /jooby
COPY --from=build /jooby/build/libs/jooby-1.0.0-all.jar app.jar
COPY conf conf
EXPOSE 8080
CMD ["java", "-jar", "app.jar", "application.env=prod"]
