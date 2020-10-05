/bin/bash: -d: invalid option
Usage:	/bin/bash [GNU long option] [option] ...
	/bin/bash [GNU long option] [option] script-file ...
GNU long options:
	--debug
	--debugger
	--dump-po-strings
	--dump-strings
	--help
	--init-file
	--login
	--noediting
	--noprofile
	--norc
	--posix
	--pretty-print
	--rcfile
	--restricted
	--verbose
	--version
Shell options:
	-ilrsD or -c command or -O shopt_option		(invocation only)
	-abefhkmnptuvxBCHP or -o option
The command '/bin/bash -asdas --verbose -eufxv -o nounset -o errexit -o noclobber -o pipefail go mod download' returned a non-zero code: 2





RUN /bin/bash -c "source /usr/local/bin/virtualenvwrapper.sh"







https://unix.stackexchange.com/a/305375/14720




https://github.com/moby/moby/issues/1799#issuecomment-124476047
RUN echo 'All of your\n\
multiline that you ever wanted\n\
into a dockerfile\n'\
>> /etc/example.conf



https://github.com/moby/moby/issues/1799#issuecomment-126083922


RUN echo $'All of your\n\
multiline that you ever wanted\n\
into a dockerfile\n'\
>> /etc/example.conf




https://github.com/moby/moby/issues/1799#issuecomment-273443979
RUN echo $'All of your\n\
multiline'"'"'s that you ever wanted\n\
> /etc/example.conf


https://github.com/moby/moby/issues/1799#issuecomment-427651274
RUN echo  $'\n\
  #!/bin/bash\n\
  foo\n\
  bar\n'\
  > /usr/local/bin/myscript.sh
RUN chmod +x /usr/local/bin/myscript.sh


https://github.com/crashsystems/gitlab-docker/blob/master/Dockerfile#L23-L31
https://github.com/docker-library/openjdk/blob/8a23a228bda7d2edeb4132fffd2d08c1e1fcf4ac/8-jdk/Dockerfile#L28-L36

# add a simple script that can auto-detect the appropriate JAVA_HOME value
# based on whether the JDK or only the JRE is installed
RUN { \
		echo '#!/bin/sh'; \
		echo 'set -e'; \
		echo; \
		echo 'dirname "$(dirname "$(readlink -f "$(which javac || which java)")")"'; \
	} > /usr/local/bin/docker-java-home \
	&& chmod +x /usr/local/bin/docker-java-home
vs

# add a simple script that can auto-detect the appropriate JAVA_HOME value
# based on whether the JDK or only the JRE is installed
RUN <<-'EOR'
	set -ex
	cat > /usr/local/bin/docker-java-home <<-'EOF'
		#!/bin/sh
		set -e

		dirname "$(dirname "$(readlink -f "$(which javac || which java)")")"
	EOF
	chmod +x /usr/local/bin/docker-java-home
EOR



RUN (cd /
# or if ^^that^^ doesn't work try
RUN /bin/sh -c "(cd /
    git clone https://github.com/
    cd /fileadmin/
    bundle install
    rake db:migrate
    bundle exec rails runner "eval(File.read 'createTestUser.rb')"
    mkdir /pending
    mkdir /live
    chmod 777 /pending
    chmod 777 /live
    )"


bash -c "$(/bin/echo -e "cat > /etc/my.config <<EOM\
\n########################################################\
\n# The file\
\n########################################################\
\na = b\
\nEOM\n")

RUN echo " \n\
KERNEL=="mali", MODE="0660", GROUP="video" \n\
KERNEL=="ump", MODE="0660", GROUP="video" \n\
" >> /etc/udev/rules.d/50-mali.rules







shopt -s checkwinsize
shopt -s cmdhist
shopt -s complete_fullquote
shopt -s extquote
shopt -s force_fignore
shopt -s globasciiranges
shopt -s hostcomplete
shopt -s interactive_comments
shopt -s progcomp
shopt -s promptvars
shopt -s sourcepath
set +o allexport
set -o braceexpand
set +o emacs
set -o errexit
set +o errtrace
set +o functrace
set -o hashall
set +o histexpand
set +o history
set +o ignoreeof
set -o interactive-comments
set +o keyword
set +o monitor
set +o noclobber
set +o noexec
set -o noglob
set +o nolog
set +o notify
set -o nounset
set +o onecmd
set +o physical
set +o pipefail
set +o posix
set +o privileged
set -o verbose
set +o vi
set -o xtrace
