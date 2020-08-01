# Chapter 2

In this tutorial we will take a look at doing feature extraction with ``AKAZE`` or ``FAST``.

This will serve as an introduction tutorial for cv.

## Create a project

Let's start by creating a project: ``cargo new --bin cv-tutorial ``. Then we are going into the newly created directory: ``cd cv-tutorial``.

## Add dependencies

Open ``cargo.toml`` and add `cv` as a dependency. The dependency section should look like this

````toml
[dependencies]
cv = "0.6.0"
````

// TODO add others

## Get some tests images

// TODO