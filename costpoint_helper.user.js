// ==UserScript==
// @name     Rustic Costpoint Helper
// @description Automates data entry in Costpoint
// @match https://praeses-cp.costpointfoundations.com/cpweb/masterPage.htm*
// @version  1
// @grant    GM.xmlHttpRequest
// @connect *
// ==/UserScript==

let dateColumnMap = null;
let chargeCodeRowMap = null;

function initializeMappings() {
  dateColumnMap = mapDatesToColumns();
  chargeCodeRowMap = mapChargeCodesToRows();
}

// A helper function to extract dates and map them to column indices
function mapDatesToColumns() {
  const dateDivs = Array.from(document.querySelectorAll("div.hdDiv"));
  let dateMap = [];

  dateDivs.forEach((div) => {
    const match = div.id.match(/hdDiv(\d+)_1/);
    if (match && div.textContent.match(/\d{2}\/\d{2}\/\d{2}/)) {
      dateMap.push({
        idNum: parseInt(match[1], 10),
        date: div.textContent.trim(),
      });
    }
  });

  dateMap.sort((a, b) => a.idNum - b.idNum);

  // we don't get the date divs for dates that aren't on screen
  // when that happens we need to calculate an offset to correct our indices
  const firstVisibleDayNum = parseInt(dateMap[0].date.split("/")[1], 10);
  // this is based on our bi-monthly pay schedule
  const payPeriodStart = firstVisibleDayNum >= 16 ? 16 : 1;
  const offset = firstVisibleDayNum - payPeriodStart;

  const dateColumnMap = {};
  dateMap.forEach((item, index) => {
    dateColumnMap[item.date] = index + 1 + offset;
  });

  return dateColumnMap;
}

function mapChargeCodesToRows() {
  // This regex matches the pattern "UDT02_ID-_" followed by any number and "_E"
  const chargeCodeIdPattern = /UDT02_ID-_(\d+)_E/;
  const inputs = document.querySelectorAll("input");
  const chargeCodeRowMap = {};

  inputs.forEach((input) => {
    const match = chargeCodeIdPattern.exec(input.id);
    if (match && match[1]) {
      // Extract the row number from the ID
      const rowNumber = parseInt(match[1], 10);
      // Map the charge code to the row number
      chargeCodeRowMap[input.value] = rowNumber;
    }
  });

  return chargeCodeRowMap;
}

function fetchTimeEntries() {
  return new Promise((resolve, reject) => {
    GM.xmlHttpRequest({
      method: "GET",
      url: "http://127.0.0.1:8001/time_entries/costpoint",
      onload: function (response) {
        console.log(response);
        if (response.status === 200) {
          resolve(JSON.parse(response.responseText));
        } else {
          reject(new Error(`HTTP error! Status: ${response.status}`));
        }
      },
      onerror: function (error) {
        reject(new Error("Error fetching time entries: " + error));
      },
    });
  });
}

async function updateCostpointWithEntries(date) {
  const timeEntries = await fetchTimeEntries(date);
  console.log("timeEntries: ", timeEntries);
  if (timeEntries && timeEntries.length > 0) {
    let updatesList = timeEntries.map((entry) => ({
      cellId: findInputCellId(entry.charge_code, entry.date),
      hours: entry.hours,
      note: entry.notes,
    }));
    await processUpdates(updatesList);
  }
}

function findInputCellId(chargeCode, date) {
  const chargeCodeRow = chargeCodeRowMap[chargeCode];
  const dayIndex = dateColumnMap[date];
  if (dayIndex !== undefined && chargeCodeRow !== undefined) {
    const inputCellId = `DAY${dayIndex}_HRS-_${chargeCodeRow}_E`;
    return inputCellId;
  }
  console.error(
    `Cell ID not found for charge code: ${chargeCode} and date: ${date}`,
  );
  return null;
}

async function example() {
  // initializing for each button press, because loading in timings are weird
  initializeMappings();
  await updateCostpointWithEntries();
}

async function processUpdates(updatesList) {
  for (let i = 0; i < updatesList.length; i++) {
    const firstUpdate = updatesList[i];
    let secondIndex = (i + 1) % updatesList.length;
    const secondUpdate = updatesList[secondIndex];

    setEntryForCell(firstUpdate.cellId, firstUpdate.hours, firstUpdate.note);
    await delay(300);
    setEntryForCell(firstUpdate.cellId, firstUpdate.hours, firstUpdate.note);
    setEntryForCell(secondUpdate.cellId, secondUpdate.hours, secondUpdate.note);

    await delay(900);
  }
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function setEntryForCell(cellId, hours, note) {
  setNoteForCell(cellId, note);
  setHoursForCell(cellId, hours);
}

function setHoursForCell(cellId, hours) {
  const cell = document.getElementById(cellId);
  if (cell) {
    cell.focus(); // Focus on the cell
    cell.value = String(hours); // Set the hours as a string
    cell.blur();
  } else {
    console.error("Input cell not found: " + cellId);
  }
}

function setNoteForCell(cellId, note) {
  // Find the cell by ID
  const cell = document.getElementById(cellId);
  if (cell) {
    // Find the span element with the 'tCommentBtn' class within the cell's parent div
    const noteSpan = cell.parentElement.querySelector(".tCommentBtn");
    if (noteSpan) {
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
          }, 100);
        } else {
          console.error("Note icon not found");
        }
      }, 100);
    } else {
      console.error("Note span not found");
    }
  } else {
    console.error("Input cell not found");
  }
}

let button = document.createElement("button");
button.textContent = "Automatically Enter Time Entries!!";
button.style.position = "fixed";
button.style.top = "10px";
button.style.right = "10px";
button.style.zIndex = "1000";

// Attach event listener to the button
button.addEventListener("click", example);

// Add the button to the body of the page
document.body.appendChild(button);
