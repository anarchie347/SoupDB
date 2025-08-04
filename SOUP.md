# SoupDB

SoupDB is a brand new paradigm of database based on something very familiar and easy to understand - a bowl of soup. This document explains the key ideas and terminology of SoupDB.

## Ingredient

Ingredients are the fundamental building blocks of a soup (as they should be). An Ingredient is a generic piece of data, in JSON form. There is no fixed schema for a soup, because it would not be logical to restrict what ingredients you could put in a soup.

## Bowl

Tables are a notoriously bad place to storw your soup, it just goes everywhere. Hence in SoupDB we have Bowls, which achieve a similar concept but are much better at holding this kind of data.

## Spoon

The only logical way to retrieve data from SoupDB is using a spoon. Spoon is the name given to a retrieva query, and they can vary in size. The important thing with spoons is you cant choose the Ingredients you retrieve, they are retrieved at random.

## Depth

All data in a bowl has a depth, with a higher number being further from the surface. When you use a spoon you can select the depth you wish to query at, which will determine which data you get back

## Mixing

Over time, Ingredients in your Soup will move between layers, with heavier (larger) Ingredients sinking faster and makybe pushing smaller ones up. This happens naturally based on time, and can also be manually invoked.

## Blending

Blending is like mixing but more violent. If you ever wanted a smooth Soup where all the Ingredients are a similar size but become unrecognisable from their original form? Well if so, blending is for you. If you blend your Soup, data is broken apart into smaller pieces and mixed around a lot between the layers. You can choose the intensity of the blending.
