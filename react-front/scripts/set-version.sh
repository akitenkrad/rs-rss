#!/bin/bash

# Get the latest git tag
LATEST_TAG=$(git tag --sort=-version:refname | head -1)

# If no tag exists, use default version
if [ -z "$LATEST_TAG" ]; then
    LATEST_TAG="v0.1.0"
fi

# Remove 'v' prefix if it exists
VERSION=${LATEST_TAG#v}

# Create or update .env file
echo "REACT_APP_VERSION=$VERSION" > .env

echo "Version set to: $VERSION"
