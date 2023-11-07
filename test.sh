set -o pipefail

var=$(podman inspect asdf 2>/dev/null | jq '.[0].Created')
echo $?
echo $var

var=$(podman inspect 4a16857245f6 2>/dev/null | jq '.[0].Created')
echo $?
echo $var
