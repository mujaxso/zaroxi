#!/bin/bash
set -e

mkdir -p assets/fonts

# Download Noto Sans Regular
curl -L -o assets/fonts/NotoSans-Regular.ttf \
    "https://fonts.gstatic.com/s/notosans/v28/o-0IIpQlx3QUlC5A4PNr5TRA.woff2"

# Download Noto Emoji Regular
curl -L -o assets/fonts/NotoEmoji-Regular.ttf \
    "https://fonts.gstatic.com/s/notoemoji/v24/bMrnmSyK7YY-MEu6aWjPDs-ar6uWaGWuob_10jw.woff2"

echo "Fonts downloaded successfully"
