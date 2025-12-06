#!/bin/sh
git add .

git commit -m "$*"

git push

git push origin master
