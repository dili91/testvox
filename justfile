alias b := docker-build
alias r := docker-run

docker-build:
    docker build . -t reporter

docker-run:
    @docker run --name reporter --rm -v "$HOME/Documents/TrueLayer/Projects/truelayer-java/build/test-results:/junit-reports" reporter