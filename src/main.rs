use std::*;

const ORBITAL_PERIOD: f64 = 365.256363004; //length of sidereal year in ephemeris days, measured in rotations per complete revolution (basically days in year)
const OBLIQUITY: f64 = 23.439281; //tilt of the planet, between -90 and 90 degrees

fn main() {
    let time: f64 = 357.0; //day in year (mod ORBITAL_PERIOD)
    let latitude: f64 = 33.3532905478; //between -90 and 90
    let _longitude: f64 = -97.8763568637; //between -180 and 180
    let _altitude: f64 = 302.0; //altitude from radius (sea level) in meters
    let declination = declination(OBLIQUITY, ORBITAL_PERIOD, time);
    let hour_angle = hour_angle(latitude, declination);
    println!("DECLINATION = {declination}");
    println!("HOUR ANGLE = {hour_angle}");
}

fn arcsin_bandpass(mut num: f64) -> f64 {
    if num < -1.0 {
        num = -90.0;
    } else if num > 1.0 {
        num = 90.0;
    } else {
        num = num.asin().to_degrees();
    }
    return num;
}

fn declination(obliquity: f64, orbital_period: f64, time: f64) -> f64 {
    //time measured in number of full rotations (days)
    let ecliptic_longitude = (360.0 * time) / orbital_period;
    let declination: f64 = -obliquity * ecliptic_longitude.to_radians().sin();
    println!("ECLIPTIC LONGITUDE = {ecliptic_longitude}");
    let x = ecliptic_longitude.to_radians().sin();
    println!("ECLIPTIC LONGITUDE SINE = {x}");
    return declination;
}

fn hour_angle(latitude: f64, declination: f64) -> f64 {
    let ltan = -latitude.to_radians().tan();
    let dtan = declination.to_radians().tan();
    println!("LATITUDE TANGENT: {ltan}");
    println!("DECLINATION TANGENT: {dtan}");
    return arcsin_bandpass(ltan * dtan);
}
