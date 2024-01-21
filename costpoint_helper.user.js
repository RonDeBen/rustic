// ==UserScript==
// @name     Rustic Costpoint Helper
// @description Automates data entry in Costpoint
// @include https://praeses-cp.costpointfoundations.com/cpweb/masterPage.htm*
// @version  1
// @grant    none
// ==/UserScript==

// A helper function to extract dates and map them to column indices
function mapDatesToColumns() {
  const dateHeaders = document.querySelectorAll("div.hdTxt");
  const dateColumnMap = {};
  let columnIndex = 1; // Start indexing from 1 for the first date

  dateHeaders.forEach((header) => {
    // Assuming the second child div contains the date
    const dateDiv = header.children[1];
    if (dateDiv && dateDiv.textContent.match(/\d{2}\/\d{2}\/\d{2}/)) {
      // Check if the text is a date
      const date = dateDiv.textContent.trim();
      dateColumnMap[date] = columnIndex.toString();
      columnIndex++; // Increment the index for the next date
    }
  });

  return dateColumnMap;
}

function findInputCellId(chargeCodeRow, date) {
  const dateColumnMap = mapDatesToColumns();
  const dayIndex = dateColumnMap[date];
  if (dayIndex !== undefined) {
    const inputCellId = `DAY${dayIndex}_HRS-_${chargeCodeRow}_E`;
    console.log("inputCellId", inputCellId);
    return inputCellId;
  }
}

// Example usage: Find the input cell for charge code row 2 on date "01/16/24"
function example() {
  const inputCellId = findInputCellId(0, "01/16/24");
  if (inputCellId) {
    setNoteForCell(inputCellId, "ron wuz here lol");
  } else {
    console.log("Input cell not found");
  }
}

function setNoteForCell(cellId, note) {
  // Find the cell by ID
  const cell = document.getElementById(cellId);
  if (cell) {
    // Find the span element with the 'tCommentBtn' class within the cell's parent div
    const noteSpan = cell.parentElement.querySelector(".tCommentBtn");
    if (noteSpan && noteIcon) {
      // Click the note span to make the note icon visible
      noteSpan.style.display = "inline"; // Make sure the span is visible before clicking
      noteSpan.click(); // Click the note span to open the note editor

      // Wait for the note editor to become visible
      setTimeout(() => {
        // Try clicking the note icon now that it should be visible
        const noteIcon = cell.nextElementSibling;
        if (noteIcon) {
          noteIcon.click();
          // Wait for the note editor to open after clicking the icon
          setTimeout(() => {
            const noteEditor = document.getElementById("expandoEdit");
            if (noteEditor) {
              // Set the note text
              noteEditor.value = note;
              // Find the "Ok" button and click it to save the note
              const okButton = document.getElementById("expandoOK");
              if (okButton) {
                okButton.click();
              } else {
                console.error("Ok button not found");
              }
            } else {
              console.error("Note editor not found");
            }
          }, 500); // Adjust this delay as necessary
        } else {
          console.error("Note icon not found");
        }
      }, 500); // Adjust this delay as necessary
    } else {
      console.error("Note span not found");
    }
  } else {
    console.error("Input cell not found");
  }
}

function findAndLogChargeCodes() {
  // This regex matches the pattern "UDT02_ID-_" followed by any number and "_E"
  const chargeCodeIdPattern = /UDT02_ID-_\d+_E/;

  // Select all input elements
  const inputs = document.querySelectorAll("input");

  // Iterate over the inputs to find the ones that match the pattern
  inputs.forEach((input) => {
    if (chargeCodeIdPattern.test(input.id)) {
      // Log the id and the value of the input element
      console.log("ID:", input.id, "Value:", input.value);
    }
  });
}

let button = document.createElement("button");
button.textContent = "Highlight Charge Codes";
button.style.position = "fixed";
button.style.top = "10px";
button.style.right = "10px";
button.style.zIndex = "1000";

// Attach event listener to the button
button.addEventListener("click", example);

// Add the button to the body of the page
document.body.appendChild(button);
