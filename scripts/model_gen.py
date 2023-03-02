#!/usr/bin/env python3

import os
from pathlib import Path
from jinja2 import Environment, FileSystemLoader
import xml.etree.ElementTree as ET

__DIR__ = Path(__file__).parent.resolve()
input_dir = f"{__DIR__}/input/"
output_dir = f"{__DIR__}/../src/models"
template_dir = f"{__DIR__}/templates/"

environment = Environment(loader=FileSystemLoader(template_dir))
model_tmpl = environment.get_template("model.txt")
mod_tmpl = environment.get_template("mod.txt")


def generate_from_xml(path: str) -> int:
    root = ET.parse(path).getroot()
    model = root.find("model")
    block = model.find("block")

    points = []
    for point in block.iterfind("point"):
        points.append({
            "id": point.attrib.get('id'),
            "type": transform_type(point.attrib.get('type')),
            "offset": point.attrib.get('offset'),
            "length": point.attrib.get('len') or 1,
            "write_access": "false"
        })

    model_id = model.attrib['id']
    model_len = model.attrib['len']
    content = model_tmpl.render(id=model_id, length=model_len, points=points)

    filename = f'model{model_id}.rs'
    with open(f'{output_dir}/{filename}', mode="w", encoding="utf-8") as results:
        results.write(content)
        return int(model_id)


def transform_type(type: str) -> str:
    match type:
        case "string": return "String"
        case "eui48": return "String"

        case "int16": return "i16"
        case "int32": return "i32"
        case "int64": return "i64"
        case "uint16": return "u16"
        case "uint32": return "u32"
        case "float32": return "f32"

        case "ipaddr": return "u32"
        case "ipv6addr": return "u128"

        case "pad": return "u16"
        case "sunssf": return "u16"

        case "enum16": return "u16"
        case "enum32": return "u32"

        case "bitfield16": return "u16"
        case "bitfield32": return "u32"

        case "acc16": return "u16"
        case "acc32": return "u32"
        case "acc64": return "u64"

        case "count": return "u16"

        case _:
            print(f'No matching transformer for type: {type}')
            return type


model_ids = []
with os.scandir(input_dir) as input_dir:
    for entry in input_dir:
        if entry.name.endswith(".xml") and entry.is_file():
            model_id = generate_from_xml(entry.path)
            model_ids.append(model_id)

with open(f'{output_dir}/mod.rs', mode="w", encoding="utf-8") as results:
    model_ids.sort()
    content = mod_tmpl.render(models=model_ids)
    results.write(content)
