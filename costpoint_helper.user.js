// ==UserScript==
// @name     Rustic Costpoint Helper
// @description Automates data entry in Costpoint
// @match https://praeses-cp.costpointfoundations.com/cpweb/masterPage.htm*
// @version  1
// @grant    GM.xmlHttpRequest
// @grant    GM_addStyle
// @connect *
// ==/UserScript==

GM_addStyle(`
  .automated-entry {
    background-color: yellow;
  }
`);

GM_addStyle(`
  .automated-entry-btn {
    display: inline-flex; /* Changed to inline-flex for better centering */
    align-items: center; /* Align items vertically */
    justify-content: center; /* Center content horizontally */
    padding: 0; /* Remove padding to use full area for the icon */
    margin-right: 5px;
    border-radius: 4px;
    background-color: #fff; /* White background */
    border: 1px solid #ccc; /* Border color */
    cursor: pointer;
    transition: background-color .15s ease-in-out, border-color .15s ease-in-out;
    width: 29px; /* Width to match other buttons */
    height: 27px; /* Height to match other buttons */
  }

  .automated-entry-btn:hover,
  .automated-entry-btn:focus {
    background-color: #BEE2EA; /* Light blue background on hover/focus */
    border-color: #13B5EA; /* Border color on hover/focus */
    outline: none;
  }

  .automated-entry-btn:active {
    box-shadow: inset 0 3px 5px rgba(0, 0, 0, .125); /* Shadow for a pressed effect */
  }

  .automated-entry-btn-img {
    width: 20px; /* Width of the icon image */
    height: 20px; /* Height of the icon image */
    background-size: contain; /* Ensure the icon fits within the dimensions */
    background-position: center; /* Center the icon within the span */
    background-repeat: no-repeat; /* Do not repeat the background image */
  }
`);

//------ initialization for the script---------

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

//------ Core Functions / Writing to Costpoint ---------

async function updateCostpointWithEntries() {
  const timeEntries = await fetchTimeEntries();
  console.log("timeEntries: ", timeEntries);
  let filteredEntries = filterEntriesForUpdate(timeEntries);

  if (filteredEntries && filteredEntries.length > 0) {
    let updatesList = filteredEntries.map((entry) => ({
      cellId: findInputCellId(entry.charge_code, entry.date),
      hours: entry.hours,
      note: entry.notes,
    }));

    const clearUpdates = clearOldAutomatedEntries(updatesList);
    await processUpdates([...updatesList, ...clearUpdates]);
  }
}

function filterEntriesForUpdate(fetchedEntries) {
  const entriesToUpdate = fetchedEntries.filter((entry) => {
    const cellId = findInputCellId(entry.charge_code, entry.date);
    const cell = document.getElementById(cellId);

    if (cell) {
      const isAutomated = cell.classList.contains("automated-entry");

      // If automated, always include
      if (isAutomated) return true;

      // If not automated, include only if both note and hours are missing
      return !doesCellHaveNote(cell) && !doesCellHaveHours(cell);
    }

    return false; // If cell not found, do not include
  });

  return entriesToUpdate;
}

function clearOldAutomatedEntries(updatesList) {
  const automatedCells = document.querySelectorAll(".automated-entry");
  const updateCellIds = updatesList.map((update) => update.cellId);
  let clearUpdates = [];

  automatedCells.forEach((cell) => {
    if (!updateCellIds.includes(cell.id)) {
      clearUpdates.push({
        cellId: cell.id,
        hours: "",
        note: "",
      });
    }
  });

  return clearUpdates;
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

//------ utility functions ---------

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

function setEntryForCell(cellId, hours, note) {
  const cell = document.getElementById(cellId);
  if (cell) {
    setNoteForCell(cell, note);
    setHoursForCell(cell, hours);
    if (hours !== "" && note !== "") {
      cell.classList.add("automated-entry");
    } else {
      cell.classList.remove("automated-entry");
    }
  } else {
    console.error("Input cell not found: " + cell);
  }
}

function setHoursForCell(cell, hours) {
  cell.focus(); // Focus on the cell
  cell.value = String(hours); // Set the hours as a string
  cell.blur();
}

function setNoteForCell(cell, note) {
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
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function doesCellHaveNote(cell) {
  const noteIcon = cell.nextElementSibling;
  return noteIcon && noteIcon.title !== "";
}

function doesCellHaveHours(cell) {
  return cell && cell.value !== "";
}

let magicImageBase64 =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAA0GVYSWZJSSoACAAAAAoAAAEEAAEAAABAAAAAAQEEAAEAAABAAAAAAgEDAAMAAACGAAAAEgEDAAEAAAABAAAAGgEFAAEAAACMAAAAGwEFAAEAAACUAAAAKAEDAAEAAAADAAAAMQECAA0AAACcAAAAMgECABQAAACqAAAAaYcEAAEAAAC+AAAAAAAAAAgACAAIADcCAAAUAAAANwIAABQAAABHSU1QIDIuMTAuMzYAADIwMjQ6MDI6MjcgMTk6MDc6MDEAAQABoAMAAQAAAAEAAAAAAAAAiq95wAAAAYNpQ0NQSUNDIHByb2ZpbGUAAHicfZE9SMNAHMVfU0WRioMdRDpkqE520SLiVKtQhAqhVmjVweTSL2jSkKS4OAquBQc/FqsOLs66OrgKguAHiLODk6KLlPi/pNAixoPjfry797h7BwjNKtOsngSg6baZSSXFXH5V7HuFgAiAWcRlZhlzkpSG7/i6R4CvdzGe5X/uzzGoFiwGBETiBDNMm3iDeHrTNjjvE4dZWVaJz4knTLog8SPXFY/fOJdcFnhm2Mxm5onDxGKpi5UuZmVTI44TR1VNp3wh57HKeYuzVq2z9j35C0MFfWWZ6zQjSGERS5AgQkEdFVRhI0arToqFDO0nffyjrl8il0KuChg5FlCDBtn1g//B726t4tSklxRKAr0vjvMxBvTtAq2G43wfO07rBAg+A1d6x19rAjOfpDc6WvQIGNoGLq47mrIHXO4AI0+GbMquFKQpFIvA+xl9Ux4YvgUG1rze2vs4fQCy1FX6Bjg4BMZLlL3u8+7+7t7+PdPu7wf7M3LdI2cQHwAADXhpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+Cjx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IlhNUCBDb3JlIDQuNC4wLUV4aXYyIj4KIDxyZGY6UkRGIHhtbG5zOnJkZj0iaHR0cDovL3d3dy53My5vcmcvMTk5OS8wMi8yMi1yZGYtc3ludGF4LW5zIyI+CiAgPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIKICAgIHhtbG5zOnhtcE1NPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvbW0vIgogICAgeG1sbnM6c3RFdnQ9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZUV2ZW50IyIKICAgIHhtbG5zOmRjPSJodHRwOi8vcHVybC5vcmcvZGMvZWxlbWVudHMvMS4xLyIKICAgIHhtbG5zOkdJTVA9Imh0dHA6Ly93d3cuZ2ltcC5vcmcveG1wLyIKICAgIHhtbG5zOnRpZmY9Imh0dHA6Ly9ucy5hZG9iZS5jb20vdGlmZi8xLjAvIgogICAgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIgogICB4bXBNTTpEb2N1bWVudElEPSJnaW1wOmRvY2lkOmdpbXA6ZDY2Y2I5MTktZWZmYy00ZTk4LTg1NWUtOTc5ZjY2MGNiMzM5IgogICB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjBjZGU4YjU2LTIzNGYtNGVmMi1hYmIwLTlkMGJkZTQwMjNjZiIKICAgeG1wTU06T3JpZ2luYWxEb2N1bWVudElEPSJ4bXAuZGlkOjE1MzVkNjhjLTA0ZTQtNDdjZi1hZjRjLWNmODdlZjBiNTI3MCIKICAgZGM6Rm9ybWF0PSJpbWFnZS9wbmciCiAgIEdJTVA6QVBJPSIyLjAiCiAgIEdJTVA6UGxhdGZvcm09IkxpbnV4IgogICBHSU1QOlRpbWVTdGFtcD0iMTcwOTA4MjQyMjkwNjcwOSIKICAgR0lNUDpWZXJzaW9uPSIyLjEwLjM2IgogICB0aWZmOk9yaWVudGF0aW9uPSIxIgogICB4bXA6Q3JlYXRvclRvb2w9IkdJTVAgMi4xMCIKICAgeG1wOk1ldGFkYXRhRGF0ZT0iMjAyNDowMjoyN1QxOTowNzowMS0wNjowMCIKICAgeG1wOk1vZGlmeURhdGU9IjIwMjQ6MDI6MjdUMTk6MDc6MDEtMDY6MDAiPgogICA8eG1wTU06SGlzdG9yeT4KICAgIDxyZGY6U2VxPgogICAgIDxyZGY6bGkKICAgICAgc3RFdnQ6YWN0aW9uPSJzYXZlZCIKICAgICAgc3RFdnQ6Y2hhbmdlZD0iLyIKICAgICAgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDoxMzVjMjVhMC0yYWI2LTRkYTctOWQ1My0zNTk0OTQyODYyMjciCiAgICAgIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkdpbXAgMi4xMCAoTGludXgpIgogICAgICBzdEV2dDp3aGVuPSIyMDI0LTAyLTI3VDE5OjA3OjAyLTA2OjAwIi8+CiAgICA8L3JkZjpTZXE+CiAgIDwveG1wTU06SGlzdG9yeT4KICA8L3JkZjpEZXNjcmlwdGlvbj4KIDwvcmRmOlJERj4KPC94OnhtcG1ldGE+CiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAKPD94cGFja2V0IGVuZD0idyI/PtRj5+QAAAAGYktHRAD/AP8A/6C9p5MAAAAJcEhZcwAACxMAAAsTAQCanBgAAAAHdElNRQfoAhwBBwL2QRXUAAAFWklEQVR42u2aa4hVVRTHf3PdzvjIV443nZC0LG+2dQayF4UWVgaZbCGMkqk0/FBBfsheRGREFEHQh4SiBPNFiNCWqQyLivKRiT1wVxMhvvI1hqlj5mN7bx/uunAYvDP3eeYcvX84nH3vPvv1P2utvdbaB2qooYaLGXVRm5DSZgiwGLgRWAus8M7+clEQoLS5DVgOjOlStQ14G1jtnT1zwRGgtKkDngFeA1Q3j+4DXgferxQRdRFY/GBgKTCriGY7gZeBld7ZdDnjJyIgAG8WuXiAscAyYIPSZlI5g/fp7dUnkqkG4H7gKHAKOA5cUmDz0cD8RDI1JJFMbUx3tJ+9EHaB0cDuEua2C3jIO7s5VhLQZfEJYAEwtQQChgKtiWTqVCKZ2pzuaI+WBChthgGd3lmfp/4yYCUwrQLDfQHM8c4ejoQRVNpcBWwBluSpbwB+qtDiAe4CvlfaTOh1ApQ2NwGbgKuB38/3jHf2NLC/wkNfKSTM6DUClDazgK+BJHBOvLxc3TilTdDpWVuFKQwCPlbaPBk6AUqbBcAaoH9OL72z+5Q2SmnzHOCATUqbcVUkAPEsG0M1gkqbFtHpIPYCLwALgZbA/8eAx+Xe1sNL2Q38COwADgF/AxmgQdaSBEbI/XIgBSz2zi7qjp1qYG4ep2XFef4fAqzqpq+fxVX+yDt7KPLRoNKmXoKWxgp12eSdPVA1T7QKfc6s4OIBro9VQkRp8wlwb4W7PS5xwgmgE5jqnT1WKQtZycWPBKZX4UUNlispv5uBb6OoAo9U0bAG0RJVG/Boie0ywJ4inm+OHAFKm1tk3y0WncCdwK9xl4Dg228DThb45md7Z78CJhQx1nVKm76RIUBpMwB4QBa00Ds7k2yqqyd85p39vIQhG0qUtqpJwH1ipZ/yzr4l/40voN2yQNkVOWZzlAhYA0z2zr4T+K8Qkf4jUF7eGztBRbYs7+w5CVJyKtGnQAkIprRXA/OAu8MkoFrhcAZ4GjhYqBh7ZzPAbGBdmCpQlaRouqM9k+5o35pIpt4F/gUmA/3OFwmmO9qXBdqdTiRTq0SacnM7BHwpXmAwXT4gkUwtSXe0H49ULJBnl1gFPJinutU7u6KH9nPyhNIzvLOfRiYW6OIU3Uz2kHMMcHs3j3+gtOnjnf0wT19zgfe6sQO9Q4DSpj9wBzARqPfOvhqofgOYUsSevlRpM1+2RQf0lX3+MeCGatqBciSgNfhmlDbrvbNbcp5aCf3dKleoW2E5u0DXtNcrQsQoYDjhYJycLodLgNLmWtHxIKZLDt4QHuoA3RsqkC/sbSN8tJA9eAlHAuQwo5XooDlsFbgHGBUhAlrCVoGc8TsN/EU2Bb5HyvvJHnAclfsxsglNAC/Jj6D+DpXyQKBe5jOc7OHGCGBkoNxE9rxvUJf5aPEjzoVFwCLgiQodUhwpQQWbgGsC13ghZy81hBgLKG36iZt7RUBMG0VsGyVBkhPXoTJWTtQBzpLN83e9nwH+AQ5LIHRQrp3ALjlKD48ApU0SmCQu70QRu7FiCMP+xigDHCB7OPobsF1c5+3e2SMVJ0BpM01C0TjgANkvSjcAG4GthUhLT9vgSzFS51HADAnEvgOeL8sPkA8XphBf6LIIEIenLsYENJVLQDPxRn25BAyMOQGd5RJwMuYE/FkuATtiTsC6cglYH+PF7yybAO/sNuCHGC7eAw8X6jL3FA0uBL4pMG9wVnz43HVE7p0SFiNhcibg+58C/iP7MWW/PJZ8IDCAbPZ4kMx5GNmDk9ynM5dK/HECmOed3VDJWOBZ4EUJN3dJ3L9XcgB7pHzYO9tJDTXUUEMN8cL/P9psVeMDVm8AAAAASUVORK5CYII=";

// -------UI Elements and Event Listeners-------
// Select the toolbar container where other buttons are located
const toolbar = document.querySelector(".tlbr");

// Create a new container for your button that matches the style of other button containers
let buttonContainer = document.createElement("div");
buttonContainer.className = "tbBtnContainer";

// Calculate and set the left position for the new button container
const leftPositionOfRightmostButton = 614; // From the rightmost button
const widthOfRightmostButton = 53; // Width of the rightmost button
const gapBetweenButtons = 5; // Space between buttons
const leftPositionForNewButton =
  leftPositionOfRightmostButton + widthOfRightmostButton + gapBetweenButtons;
buttonContainer.style.left = `${leftPositionForNewButton}px`;

// Create the button
const button = document.createElement("div");
button.className = "automated-entry-btn";

// Create the icon span and add the base64 background image
const iconSpan = document.createElement("span");
iconSpan.className = "automated-entry-btn-img";
iconSpan.style.backgroundImage = `url(${magicImageBase64})`;

// Add the icon span to the button
button.appendChild(iconSpan);

// Add the button to the button container
buttonContainer.appendChild(button);

// Append the button container to the toolbar
toolbar.appendChild(buttonContainer);

// Attach an event listener to the button for click events
button.addEventListener("click", automateEntries);

async function automateEntries() {
  console.log("I was pressed");
  // initializing for each button press, because loading in timings are weird
  initializeMappings();
  await updateCostpointWithEntries();
}
