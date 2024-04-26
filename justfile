alias db := docker-build
alias dr := docker-run

docker-build:
    docker build . -t reportly

docker-run:
    @docker run --rm -v "$HOME/Documents/TrueLayer/Projects/truelayer-java/build/test-results:/junit-reports" reportly