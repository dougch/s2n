FROM ubuntu:14.04


ENV TERM=ansi
RUN apt-get -y install software-properties-common
RUN apt-add-repository ppa:brightbox/ruby-ng
RUN apt-get update && apt-get install -y libssl-dev python-pip ruby2.3
RUN gem install bundler

COPY codebuild codebuild
RUN codebuild/bin/install_sidetrail_dependencies.sh && \
   mkdir -p /sidetrail-install-dir && \
   codebuild/bin/install_sidetrail.sh /sidetrail-install-dir

CMD ['s2n/codebuild/bin/run_sidetrail.sh','/sidetrail-install-dir','/s2n']
