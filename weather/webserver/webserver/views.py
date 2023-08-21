import json
from django.shortcuts import render
from django.http import JsonResponse

def get_json(request):
    with open('/rems/readings/weather.json', 'r') as json_file:
        data = json.load(json_file)

    temperature = data['data']['BMP280']['Temperature']
    pressure = data['data']['BMP280']['Pressure']
    co2 = data['data']['SGP30']['C02']
    tvoc = data['data']['SGP30']['TVOC']

    context = {
        'temperature': round(temperature, 1),
        'pressure': round(pressure, 1),
        'co2': round(co2, 1),
        'tvoc': round(tvoc, 1),
    }

    return JsonResponse(context)

def home(request):
    return render(request, 'index.html')
