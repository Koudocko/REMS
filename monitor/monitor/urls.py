"""
URL configuration for monitor project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/4.2/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""
from django.contrib import admin
from django.urls import path
from . import views

urlpatterns = [
    # Root endpoint of the server displays static and template files
    path('', views.home, name='home'),
    # get_json endpoint is used by Ajax to update HTML with JSON
    path('get_json/', views.get_json, name='update_ui'),
    path('start_service/', views.start_service, name='start_service'),
    path('stop_service/', views.stop_service, name='stop_service'),
    path('enable_service/', views.enable_service, name='enable_service'),
    path('disable_service/', views.disable_service, name='disable_service'),
]

