import re, subprocess
from django.shortcuts import render
from django.http import JsonResponse, HttpResponse

def service_info(service_name, service_status):
    # Regex matches
    loaded_pattern = re.compile(r'Loaded:.*;\s*(\w+);')
    active_pattern = re.compile(r'Active:\s*(\S+)\s*')

    loaded_match = loaded_pattern.search(service_status)
    active_match = active_pattern.search(service_status)

    loaded_status = loaded_match.group(1) if loaded_match else None
    active_status = active_match.group(1) if active_match else None

    # Dictionary of service
    service_info = {
        'service': service_name,
        'runtimeStatus': active_status,
        'startupStatus': loaded_status
    }

    return service_info

# Start service end point
def start_service(request):
    service = request.POST.get('service')
    subprocess.run(['systemctl', 'start', service])
    return HttpResponse()

# Stop service endpoint
def stop_service(request):
    service = request.POST.get('service')
    subprocess.run(['systemctl', 'stop', service])
    return HttpResponse()

# Enable service endpoint
def enable_service(request):
    service = request.POST.get('service')
    subprocess.run(['systemctl', 'enable', service])
    return HttpResponse()

# Disable service endpoint
def disable_service(request):
    service = request.POST.get('service')
    subprocess.run(['systemctl', 'disable', service])
    return HttpResponse()

# Retuns list of services from services.txt at endpoint get_json for Ajax to use
def get_json(request):
    # Services list
    with open('/rems/files/monitor/monitor/static/services/services.txt', 'r') as text_file:
        services = text_file.read()

    services_json = []
    for service in services.split():
        service_status = subprocess.run(['systemctl', 'status', service], capture_output=True, text=True).stdout
        services_json.append(service_info(service, service_status))

    # print(services_json)
    return JsonResponse(services_json, safe=False)

# Renders the website at / 
def home(request):
    return render(request, 'index.html')
