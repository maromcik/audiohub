podman login cerit.io
podman build -t cerit.io/roman_alexander_mariancik/audiohub-recommender-image ./recommender-server/.
podman build -t cerit.io/roman_alexander_mariancik/audiohub-image .
podman push cerit.io/roman_alexander_mariancik/audiohub-recommender-image:latest
podman push cerit.io/roman_alexander_mariancik/audiohub-image:latest