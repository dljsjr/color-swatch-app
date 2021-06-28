#! /usr/bin/env python3
import fire
import csv
import colorsys
import urllib.request
import requests
import random
import time

CSV_URL = "https://unpkg.com/color-name-list/dist/colornames.bestof.csv"


def get_color_dict():
    with urllib.request.urlopen(CSV_URL) as response:
        response = response.read().decode("utf-8")
        return csv.DictReader(response.splitlines())


def color_row_to_json(row):
    hex_code = row['hex'].lstrip('#')
    (red, green, blue) = (
        int(hex_code[0:2], 16), int(hex_code[2:4], 16), int(hex_code[4:6], 16))
    (hue, sat, val) = colorsys.rgb_to_hsv(
        red / 255.0, green / 255.0, blue / 255.0)
    color_name = row['name']

    value = {"hue": hue, "sat": sat, "val": val}

    ret = {"id": 0, "name": color_name, "value": value}

    return ret


def populate_database(color_rows, url, number):
    for (idx, row) in enumerate(color_rows):
        color = color_row_to_json(row)

        response = requests.post(
            url, json=color)

        print(response)

        time.sleep(0.1)

        # try:
        #     cursor.execute(
        #         "INSERT INTO colors ( name, value ) VALUES ( %s, ((%s)::colorPart,(%s)::colorPart,(%s)::colorPart)::colorHSV )", (color_name, hue, sat, val))
        # except psycopg2.errors.UniqueViolation:
        #     print(
        #         F"Color {color_name} already exists in DB, skipping")
        #     db_conn.commit()
        # except:
        #     print(
        #         F"Unexpected database error while generating colors: {sys.exc_info()[0]}; aborting color generation")
        #     break

        if idx >= (number - 1):
            break


class ColorGenerator(object):
    """Generate the color swatch list using https://github.com/meodai/color-names"""

    def default(self, url, number=100):
        populate_database(get_color_dict(), url, number)

    def random(self, url, number=100):
        colors = list(get_color_dict())
        random.shuffle(colors)

        populate_database(colors, url, number)


if __name__ == '__main__':
    fire.Fire(ColorGenerator)
