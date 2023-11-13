// Utilizes ajax to make a web request to the django backend at endpoint get_json and receive JSON payload data
function updateUI() {
    $.ajax({
        url: '/get_json/',
        dataType: 'json',
        success: function(services) {
          $('#main-table').html(`
            <table>
              <tr>
                <td>Service</td>
                <td>Runtime Status</td>
                <td>Startup Status</td>
                <td>Operations</td>
              </tr>
            </table>
          `);

          for (var i = 0; i < services.length; i++) {
            var service = services[i];

            // Row data columns
            var newRow = $('<tr>');
            newRow.append('<td>' + service.service + '</td>');
            newRow.append('<td>' + service.runtimeStatus + '</td>');
            newRow.append('<td>' + service.startupStatus + '</td>');

            // Action buttons
            var enableButton = $('<button type="button" onclick="sendServiceAction(\'' + service.service + '\', \'enable_service\')">Enable</button>');
            newRow.append(enableButton);

            var disableButton = $('<button type="button" onclick="sendServiceAction(\'' + service.service + '\', \'disable_service\')">Disable</button>');
            newRow.append(disableButton);

            var startButton = $('<button type="button" onclick="sendServiceAction(\'' + service.service + '\', \'start_service\')">Start</button>');
            newRow.append(startButton);

            var stopButton = $('<button type="button" onclick="sendServiceAction(\'' + service.service + '\', \'stop_service\')">Stop</button>');
            newRow.append(stopButton);

            $('#main-table').append(newRow);
          }
        }
    });
}

// Send service action notification to django backend
function sendServiceAction(serviceName, action) {
    $.ajax({
        url: '/' + action + '/',
        method: 'POST',
        data: { service: serviceName }
    });
}

setInterval(updateUI, 1000);
window.onload = updateUI
