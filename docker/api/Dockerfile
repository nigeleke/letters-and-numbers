FROM eclipse-temurin:latest

WORKDIR /usr/local/bin/lettersandnumbers/

COPY ./target/scala-3.1.1/ .

EXPOSE 80

ENTRYPOINT java -jar /usr/local/bin/lettersandnumbers/api-assembly-0.1.0-SNAPSHOT.jar
