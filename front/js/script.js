const apiUrl = 'http://localhost:8080/call_function';

function sendDataToServer(inputString) {
  const requestData = {
    data: inputString.split(' ')
  };

  return fetch(apiUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(requestData),
  })
    .then(response => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return response.json();
  });
}

function onClick() {
  const inputString = document.getElementById('inputText').value;

  if (!inputString) {
    alert("Invalid input. Please enter three words separated by spaces.");
    return;
  }
  sendDataToServer(inputString)
    .then(data => {
      console.log(data);
      alert("Data sent successfully");
    })
    .catch(error => {
      console.error('Error executing POST request:', error);
      alert(`Error sending data: ${error.message}`);
  });
}

document.addEventListener('DOMContentLoaded', function () {
  const myButton = document.getElementById('MyButton');
  myButton.addEventListener('click', onClick);
});
