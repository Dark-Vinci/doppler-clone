#!/bin/sh

cd ../

echo "ABOUT TO ADD LATEST CHANGES TO STAGING AREA"

git add .

echo "ABOUT TO COMMIT PROJECT"

git commit -am "chore: $1"

echo "🤞PROJECT has been committed🤞"

git push -u origin main

echo "🚀🚀🚀PROJECT PUSHED TO GITHUB🚀🚀🚀"
