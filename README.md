# Climate Sensor

This proyect aims to keep track of **geolocational data** collected by
*arduino* devices, such as **temperature**, **humidity** and 
**GPS coordinates**, by using a *REST api*. All of that data can then be
viewed by the target users by means of a *front-end web app*, which was
designed on par with the REST service.

## Functional Requirements

## Non-Functional Requirements

## Technologies used
### Backend
For the backend I used the ***Rust*** programming language, mainly
because of how **fast** programs written in it can be, and how it's
*ownership system* makes it pretty much the **safest** programming
language out there. *(If you want to know more about ***Rust*** and the
benefits of using it, check out their web page right 
[here](https://www.rust-lang.org/))*.

I also used ***Rocket***, the best web framework available for
***Rust***. As said by it's developers, ***Rocket*** "makes it simple to
write *fast*, *secure* web applications without sacrificing
*flexibility*, *usability*, or *type safty*." Just for those reasons, it
seemed as a no-brainer to me to choose it.

Finally, all of the data collected by the REST service is stored in a
***MongoDB*** database deployed using MongoDB Atlas, a web service that
lets you deploy fully managed *MongoDB clustes* across *AWS*, *Azure*, or
*GCP*. 

### Frontend
For the frontend I used ***Vue.js***, a ***javascript*** framework for
building user interfaces and *single-page applications*. I chose it for
its *simplicity* and *ease of use*, mainly because prior to this
project, I had almost no experience with frontend development (I have
never really liked frontend programming that much to be honest).

## REST Api Specification
The REST api is composed of the folowing services:

- For **requesting the data** (GET) collected by the sensors:
  - **Uri:** /climate/data.

- For **posting data** (POST) collected by a sensor: 
  - **Uri:** /climate/sensor.
  - **Expected parameters:** (name: type)
    - temperature: *float*.
    - humidity: *float*.
    - latitude: *float*.
    - longitude: *float*.
    - mac_addr: *string*.

## REST Api Service Authentication
For the service authentication, I made it so that every entry posted by
a sensor must have an associated **MAC address** that identifies the
sensor that is making that request, so that in the future every single
document found in the database may be *traced back* to the sensor that
posted it, if needed. *(I know it is not the most secure way of doing it,
but it gets the job done in this case, taking into account the time
constraints that I had).*

## Main Problems Found While Developing
The main problem that I had while developing this proyect was that,
as I said before, prior to this proyect I hadn't really had a lot
of experience with frontend programming, so when I had to do the
user interface, I didn't even know where to start.

Fortunately, there are *a lot* of tutorials online about UI development,
so after watching and reading a couple of them, I was ready to program
mine.

In the end, Doing it wasn't that hard, but the fact that it took me quite
some time to actually be able to make a decent user interface made it a
problem.

## Things leart During The Development Process
I learnt a fair amount of information about frontend
programming (specially about how *single-page apps* work), and though I
still don't really like it, now I can defend
myself in that terrirory if I ever have to.