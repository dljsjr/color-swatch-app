#! /usr/bin/env python3
import fire
import csv
import colorsys
import urllib.request
import psycopg2
import sys

CSV_URL = "https://unpkg.com/color-name-list/dist/colornames.bestof.csv"


def get_color_dict():
    with urllib.request.urlopen(CSV_URL) as response:
        response = response.read().decode("utf-8")
        return csv.DictReader(response.splitlines())


def color_row_to_record(row):
    hex_code = row['hex'].lstrip('#')
    (red, green, blue) = (
        int(hex_code[0:2], 16), int(hex_code[2:4], 16), int(hex_code[4:6], 16))
    (hue, sat, val) = colorsys.rgb_to_hsv(
        red / 255.0, green / 255.0, blue / 255.0)
    color_name = row['name']
    return color_name, (hue, sat, val)


def create_database_connection():
    db_conn = psycopg2.connect(
        "dbname=ccolors user=postgres password=postgrespassword host=localhost")
    cursor = db_conn.cursor()
    return db_conn, cursor


def close_database_connection(db_conn, cursor):
    db_conn.commit()
    cursor.close()
    db_conn.close()


class ColorGenerator(object):
    """Generate the color swatch list using https://github.com/meodai/color-names"""

    def default(self, number=100):
        db_conn, cursor = create_database_connection()

        for (idx, row) in enumerate(get_color_dict()):

            color_name, (hue, sat, val) = color_row_to_record(row)

            try:
                cursor.execute(
                    "INSERT INTO colors ( name, value ) VALUES ( %s, ((%s)::colorPart,(%s)::colorPart,(%s)::colorPart)::colorHSV )", (color_name, hue, sat, val))
            except psycopg2.errors.UniqueViolation:
                print(
                    F"Color {color_name} already exists in DB, skipping")
                db_conn.commit()
            except:
                print(
                    F"Unexpected database error while generating colors: {sys.exc_info()[0]}; aborting color generation")
                close_database_connection(db_conn, cursor)
                break

            if idx >= (number - 1):
                close_database_connection(db_conn, cursor)
                break

    def random(self, number=100):
        print(f"Generating {number} random colors")


if __name__ == '__main__':
    fire.Fire(ColorGenerator)
