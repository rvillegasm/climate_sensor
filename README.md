# Climate Sensor

This proyect aims to keep track of **geolocational data** collected by
*arduino* devices, such as **temperature**, **humidity** and 
**GPScoordinates**, by using a *REST api*. All of that data can then be
viewed by the target users by means of a *front-end web app*, which was
designed on par with the REST service.

## Functional Requirements

## Non-Functional Requirements

## Technologies used

### Backend
For the backend I used the `Rust` programming language, mainly
because of how **fast** programs written in it can be, and how it's
*ownership system* makes it pretty much the **safest** programming
language out there. *(If you want to know more about `Rust` and the
benefits of using it, check out their web page right 
[here](https://wwwrust-lang.org/))*.

I also used `Rocket`, the best web framework available for `Rust`. As
said by it's developers, `Rocket` "makes it simple to write *fast*
*secure* web applications without sacrificing *flexibility*,
*usability*, or *type safty*." Just for those reasons, it seemed as a
no-brainer to me to choose it.

Finally, all of the data collected by the REST service is stored in a
`MongoDB` database deployed using MongoDB Atlas, a web service that lets
you deploy fully managed *MongoDB clustes* across *AWS*, *Azure*, or
*GCP*. 

### Frontend
For the frontend I used `Vue.js`, a `javascript` framework for building
user interfaces and *single-page applications*. I chose it for its
*simplicity* and *ease of use*, mainly because prior to this project, I
had almost no experience with frontend development (I have never really
liked frontend programming that much to be honest).

## REST Api Specification

## REST Api Service Authentication

## Main Problems Found While Deviloping

## Things leart During The Development Process

## 