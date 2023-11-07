#!/usr/bin/bash

IMAGE="$1"
IDS=$(podman ps -a --format "table {{.ID}}\t{{.Image}}\t{{.CreatedAt}}" | grep "$IMAGE" | sort -k 3 -r | awk 'NR>1 {print $1}')
for id in ${IDS[@]}; do
  podman stop $id >/dev/null
  echo "rm: $(podman rm $id)"
done

echo "Cleaning complete: $IMAGE"
