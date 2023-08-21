import json
from django.shortcuts import render
from django.http import JsonResponse

def compute_aqi(tvoc, co2):
    levels = ['Low risk', 'Moderate risk', 'High risk', 'Very high risk', 'Severe risk', 'Hazardous risk']
    tvoc_ranges = [100, 200, 300, 400, 500, float('inf')]
    co2_ranges = [800, 1000, 1500, 2000, 2500, float('inf')]

    vidx, cidx = 0, 0
    for i, value in enumerate(tvoc_ranges):
        if tvoc <= value:
            vidx = i
            break;
    for j, value in enumerate(co2_ranges):
        if co2 <= value:
            cidx = j
            break;

    return levels[max(cidx, vidx)]

def get_json(request):
    with open('/rems/readings/weather.json', 'r') as json_file:
        data = json.load(json_file)

    temperature = data['data']['BMP280']['Temperature']
    pressure = data['data']['BMP280']['Pressure']
    co2 = data['data']['SGP30']['C02']
    tvoc = data['data']['SGP30']['TVOC']
    aqi = compute_aqi(tvoc, co2)

    context = {
        'temperature': round(temperature, 1),
        'pressure': round(pressure, 1),
        'co2': round(co2, 1),
        'tvoc': round(tvoc, 1),
        'aqi': aqi
    }

    return JsonResponse(context)

def home(request):
    return render(request, 'index.html')
