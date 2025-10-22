#! /bin/bash

set -e

git push dokku-api master --force
git push dokku-web master --force
