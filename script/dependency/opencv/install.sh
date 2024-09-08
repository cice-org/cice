cur_dir="$(dirname "$0")"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	os_family="Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
	os_family="macOS"
elif [[ "$OSTYPE" == "cygwin" || "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
	os_family="Windows"
elif [[ "$OSTYPE" == "freebsd"* ]]; then
	exit "FreeBSD is not supported"
else
	exit "Unknown OS: $OSTYPE"
fi

if [[ "$os_family" == "Linux" ]]; then
	# free up disk space in Github Actions image: https://github.com/actions/runner-images/issues/2840
	sudo rm -rf /usr/share/dotnet /opt/ghc /usr/local/share/boost
	"$cur_dir/install-ubuntu.sh"
elif [[ "$os_family" == "macOS" ]]; then
	"$cur_dir/install-macos-brew.sh"
elif [[ "$os_family" == "Windows" ]]; then
	"$cur_dir/install-windows-choco.sh"
fi