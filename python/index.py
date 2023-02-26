import pyo3_async
import asyncio
import xml.etree.ElementTree as ET

async def rust_rss():
    res = await pyo3_async.get_rss()
    root = ET.fromstring(res)
    for entry in root.findall("{http://www.w3.org/2005/Atom}entry"):
        print(entry.find("{http://www.w3.org/2005/Atom}title").text)

asyncio.run(rust_rss())