#! /usr/bin/env python3
import fire
import csv
import colorsys
import urllib.request
import psycopg2

CSV_URL = "https://unpkg.com/color-name-list/dist/colornames.bestof.csv"


class ColorGenerator(object):
    """Generate the color swatch list using https://github.com/meodai/color-names"""

    def default(self, number=100):
        with urllib.request.urlopen(CSV_URL) as response:
            response = response.read().decode("utf-8")

            db_conn = psycopg2.connect(
                "dbname=ccolors user=postgres password=postgrespassword host=localhost")
            cursor = db_conn.cursor()

            for (idx, line) in enumerate(csv.DictReader(response.splitlines())):
                hex_code = line['hex'].lstrip('#')
                red = int(hex_code[0:2], 16)
                green = int(hex_code[2:4], 16)
                blue = int(hex_code[4:6], 16)

                (hue, sat, val) = colorsys.rgb_to_hsv(
                    red / 255.0, green / 255.0, blue / 255.0)

                try:
                    cursor.execute(
                        "INSERT INTO colors ( name, value ) VALUES ( %s, ((%s)::colorPart,(%s)::colorPart,(%s)::colorPart)::colorHSV )", (line['name'], hue, sat, val))
                except psycopg2.errors.UniqueViolation:
                    print(
                        F"Color {line['name']} already exists in DB, skipping")
                    db_conn.commit()

                if idx >= (number - 1):
                    db_conn.commit()
                    cursor.close()
                    db_conn.close()
                    break

    def random(self, number=100):
        print(f"Generating {number} random colors")


if __name__ == '__main__':
    fire.Fire(ColorGenerator)
