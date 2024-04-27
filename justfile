alias db := docker-build
alias dr := docker-run

docker-build:
    docker build . -t reportly

docker-run:
    @docker run --rm -v "./test-reports/basic:/home/reportly/test-reports/" reportly