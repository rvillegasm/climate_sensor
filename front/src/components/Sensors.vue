<template>
    <div class="container">
        <h3>Data:</h3>
        <table class="table">
            <thead>
                <tr>
                    <th scope="col">Temperature</th>
                    <th scope="col">Humidity</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="sensor in sensors" v-bind:key="sensor._id.$oid">
                    <td>{{sensor.temperature}}</td>
                    <td>{{sensor.humidity}}</td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script>
import axios from 'axios';

export default {
    name: 'Sensors',
    data() {
        return {
            sensors: null,
        };
    },
    created() {
        axios
            .get("http://localhost:8000/climate/data")
            .then(res => {
                this.sensors = res.data;
            })
    }
}
</script>

<style>
h3 {
    margin-bottom: 5%;
}
</style>

