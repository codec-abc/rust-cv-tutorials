# Doing features matching

In this tutorial we will be doing our second Rust-CV program. Our goal will be to run the Akaze features extractor and display its result.

By oversimplifying quite a lot, features are points in a image that are in interest to us because of their values and their neighbors values. Usually, we try to found points that have specific patterns. The idea is that each feature is quite unique so when comparing pictures if we found the same features on 2 different pictures we can be pretty confident that we are seeing the same object.

If you want a more precise and accurate description of features, reading the OpenCV documentation about [features](https://docs.opencv.org/master/df/d54/tutorial_py_features_meaning.html) or Wikipedia Feature [page](https://en.wikipedia.org/wiki/Feature_(computer_vision)) is recommended.