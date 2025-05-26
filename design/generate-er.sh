#!/bin/bash
set -eu
sqlant ${DATABASE_URL} --legend -e -n --output plantuml > design/er.pu