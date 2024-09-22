May the rust god shine upon you,

First requirement is a an event dump with the format:
p1, p2, time, p1.x, p1.y, p1.z, p1.vx, p1.vy, p1.vz, p1.r, p2.x, p2.y, p2.z, p2.vx, p2.vy, p2.vz, p2.r
(not that the "." will be removed as it casues code issues)

Next one needs to run the "to.csv.py" file and enter the file path of the eventdump,
then give a name of the desired csv file.
    "to.csv.py" is depedent on the following python packages:
        1) Pyspark
        2) Pandas

Next one will run the timing file from "main.rs". It will prompt you to enter the csv file path
do so and it will return the time elasped from testing the binary heap.
