#!/usr/bin/python
# Carson Boden / October 2016
# This Python program searches Epicurious for recipes with the input ingredients
# It then scrubs the output and returns recipes and their instructions
import os

import requests
from lxml import html

print("Enter list of possible ingredients: [Type 'q' to quit]")
ingredient = input()
ingredients = []

while ingredient != "q" and ingredient != "":
    ingredients.append(ingredient)
    ingredient = input()

# print(ingredients)

# url = "http://www.google.com/#q="
url = "http://www.epicurious.com/search/"

for ingredient in ingredients:
    url = url + ingredient + "%20"
url = url + "recipe?content=recipe"

print(url)

page = requests.get(url)
tree = html.fromstring(page.content)

# This will create a list of recipe options:
titles = tree.xpath(
    'body/span/div/span/section/div/article/a[@class="view-complete-item"]'
)

for i in range(0, len(titles)):
    print("[" + str(i).zfill(2) + "]:", titles[i].get("title"))

if len(titles) == 0:
    print("No recipes found. Try reducing number of ingredients.")
    exit()

selected = False
while not selected:
    try:
        selection = int(input("Please select an option: "))
        selected = True
    except:
        print("Invalid selection.")

os.system("clear")
url = "http://www.epicurious.com/" + titles[selection].get("href")
print(url)

page = requests.get(url)
tree = html.fromstring(page.content)

subsections = tree.xpath(
    'body/div/div/div/div/div/div/div/div/ol/li[@class="ingredient-group"]'
)
ingredients = tree.xpath(
    'body/div/div/div/div/div/div/div/div/ol/li/ul[@class="ingredients"]/*'
)
steps = tree.xpath(
    'body/div/div/div/div/div/div/div/div/ol/li/ol[@class="preparation-steps"]/*'
)

print("\n=*=*=", titles[selection].get("title").upper(), "=*=*=\n")
print("INGREDIENTS:")
for i in range(0, len(ingredients)):
    for ingredient in subsections[i]:
        print(ingredient)

# for i in range(0, len(ingredients)):
#     print('[' + str(i).zfill(2) + ']:', ingredients[i].text)

print("\nSTEPS:")
for i in range(0, len(steps)):
    print("[" + str(i).zfill(2) + "]:", steps[i].text.strip())
