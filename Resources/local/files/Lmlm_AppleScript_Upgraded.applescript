(*
    LMLM Downloads Organizer — upgraded AppleScript
    Author: LMLM / Qubuhub
    Purpose:
      - Organize top-level Downloads files into PDFs and Images folders
      - Merge every eligible PDF into a timestamped output
      - Create a timestamped ZIP outside the Images folder
      - Produce a timestamped activity report
      - Preview changes before execution
    Requirements:
      - macOS
      - Optional: `pdfunite` (Poppler) for PDF merging
*)

property pdfFolderName : "PDFs"
property imageFolderName : "Images"

on run
    set downloadsFolder to (path to downloads folder)
    set downloadsPOSIX to POSIX path of downloadsFolder
    set pdfFolderPOSIX to downloadsPOSIX & pdfFolderName & "/"
    set imageFolderPOSIX to downloadsPOSIX & imageFolderName & "/"

    set choice to button returned of (display dialog ¬
        "LMLM Downloads Organizer" & return & return & ¬
        "This run will inspect files directly inside Downloads." & return & ¬
        "It will not scan subfolders." & return & return & ¬
        "Choose Preview only to see the planned changes first." ¬
        buttons {"Cancel", "Preview only", "Organize now"} ¬
        default button "Preview only" with title "LMLM")

    set executeChanges to (choice is "Organize now")
    set runStamp to do shell script "/bin/date +%Y%m%d-%H%M%S"
    set mergedOutputPOSIX to pdfFolderPOSIX & "merged-" & runStamp & ".pdf"
    set zipOutputPOSIX to downloadsPOSIX & "images-" & runStamp & ".zip"
    set reportOutputPOSIX to downloadsPOSIX & "Lmlm_Report-" & runStamp & ".txt"

    set plannedPDFs to {}
    set plannedImages to {}
    set movedPDFs to {}
    set movedImages to {}
    set warnings to {}

    try
        do shell script "/bin/mkdir -p " & quoted form of pdfFolderPOSIX & " " & quoted form of imageFolderPOSIX

        tell application "Finder"
            set sourceFiles to every file of downloadsFolder
        end tell

        repeat with sourceFile in sourceFiles
            tell application "Finder"
                set sourceName to name of sourceFile
                set sourcePOSIX to POSIX path of (sourceFile as alias)
            end tell

            set lowerName to do shell script "/usr/bin/printf %s " & quoted form of sourceName & " | /usr/bin/tr '[:upper:]' '[:lower:]'"

            if lowerName ends with ".pdf" then
                set end of plannedPDFs to sourceName
                if executeChanges then
                    set destinationPOSIX to my uniqueDestination(pdfFolderPOSIX, sourceName)
                    do shell script "/bin/mv " & quoted form of sourcePOSIX & " " & quoted form of destinationPOSIX
                    set end of movedPDFs to (do shell script "/usr/bin/basename " & quoted form of destinationPOSIX)
                end if
            else if lowerName ends with ".jpg" or lowerName ends with ".jpeg" or lowerName ends with ".png" or lowerName ends with ".heic" or lowerName ends with ".webp" then
                set end of plannedImages to sourceName
                if executeChanges then
                    set destinationPOSIX to my uniqueDestination(imageFolderPOSIX, sourceName)
                    do shell script "/bin/mv " & quoted form of sourcePOSIX & " " & quoted form of destinationPOSIX
                    set end of movedImages to (do shell script "/usr/bin/basename " & quoted form of destinationPOSIX)
                end if
            end if
        end repeat

        set reportText to "LMLM Downloads Organizer" & return & ¬
            "Mode: " & choice & return & ¬
            "Generated: " & (current date as text) & return & ¬
            "Downloads: " & downloadsPOSIX & return & return & ¬
            "PDF files found: " & (count of plannedPDFs) & return & ¬
            my listToText(plannedPDFs) & return & return & ¬
            "Image files found: " & (count of plannedImages) & return & ¬
            my listToText(plannedImages) & return

        if executeChanges then
            -- Merge all PDFs in the destination folder, excluding prior LMLM outputs.
            set pdfPaths to {}
            tell application "Finder"
                set destinationPDFs to every file of folder (POSIX file pdfFolderPOSIX as alias)
            end tell

            repeat with pdfFile in destinationPDFs
                tell application "Finder"
                    set pdfName to name of pdfFile
                    set pdfPOSIX to POSIX path of (pdfFile as alias)
                end tell
                set lowerPDFName to do shell script "/usr/bin/printf %s " & quoted form of pdfName & " | /usr/bin/tr '[:upper:]' '[:lower:]'"
                if lowerPDFName ends with ".pdf" and lowerPDFName does not start with "merged-" then
                    set end of pdfPaths to pdfPOSIX
                end if
            end repeat

            set pdfPaths to my sortTextList(pdfPaths)
            if (count of pdfPaths) > 1 then
                try
                    do shell script "/usr/bin/which pdfunite >/dev/null 2>&1"
                    set mergeCommand to "pdfunite"
                    repeat with pdfPath in pdfPaths
                        set mergeCommand to mergeCommand & " " & quoted form of (contents of pdfPath)
                    end repeat
                    set mergeCommand to mergeCommand & " " & quoted form of mergedOutputPOSIX
                    do shell script mergeCommand
                    set reportText to reportText & return & "Merged PDF: " & mergedOutputPOSIX & return & "PDFs merged: " & (count of pdfPaths) & return
                on error
                    set end of warnings to "PDF merge skipped: pdfunite is not installed or the merge failed. Install Poppler to enable PDF merging."
                end try
            else if (count of pdfPaths) is 1 then
                set reportText to reportText & return & "PDF merge skipped: only one PDF is available." & return
            else
                set reportText to reportText & return & "PDF merge skipped: no eligible PDFs were found." & return
            end if

            -- Archive the Images folder to Downloads, never inside the folder being zipped.
            if (count of plannedImages) > 0 or (count of movedImages) > 0 then
                try
                    do shell script "/usr/bin/ditto -c -k --sequesterRsrc --keepParent " & quoted form of imageFolderPOSIX & " " & quoted form of zipOutputPOSIX
                    set reportText to reportText & return & "Image archive: " & zipOutputPOSIX & return
                on error archiveError
                    set end of warnings to "Image archive failed: " & archiveError
                end try
            else
                set reportText to reportText & return & "Image archive skipped: no new images were found." & return
            end if

            set reportText to reportText & return & "Files moved to PDFs: " & (count of movedPDFs) & return & my listToText(movedPDFs) & return & ¬
                "Files moved to Images: " & (count of movedImages) & return & my listToText(movedImages) & return
        else
            set reportText to reportText & return & "PREVIEW ONLY — no files were moved, merged, or archived." & return
        end if

        if (count of warnings) > 0 then
            set reportText to reportText & return & "Warnings:" & return & my listToText(warnings) & return
        end if

        do shell script "/usr/bin/printf %s " & quoted form of reportText & " > " & quoted form of reportOutputPOSIX

        if executeChanges then
            set warningSummary to ""
            if (count of warnings) > 0 then
                set warningSummary to return & "Warnings: " & (count of warnings)
            end if
            display dialog "Organization run finished." & return & return & ¬
                "PDFs moved: " & (count of movedPDFs) & return & ¬
                "Images moved: " & (count of movedImages) & return & ¬
                "Report: " & reportOutputPOSIX & warningSummary ¬
                buttons {"OK"} default button "OK" with title "LMLM"
        else
            display dialog "Preview created. No files were changed." & return & return & ¬
                "PDFs detected: " & (count of plannedPDFs) & return & ¬
                "Images detected: " & (count of plannedImages) & return & ¬
                "Report: " & reportOutputPOSIX ¬
                buttons {"OK"} default button "OK" with title "LMLM"
        end if

    on error errorMessage number errorNumber
        try
            do shell script "/usr/bin/printf %s " & quoted form of ("LMLM Organizer error " & errorNumber & ": " & errorMessage) & " > " & quoted form of reportOutputPOSIX
        end try
        display alert "LMLM Organizer stopped" message (errorMessage & " (" & errorNumber & ")") as critical
    end try
end run

on uniqueDestination(destinationFolder, originalName)
    set candidatePath to destinationFolder & originalName
    set counter to 2
    repeat
        try
            do shell script "/usr/bin/test -e " & quoted form of candidatePath
            set nameParts to my splitFilename(originalName)
            set candidatePath to destinationFolder & item 1 of nameParts & " (" & counter & ")" & item 2 of nameParts
            set counter to counter + 1
        on error
            return candidatePath
        end try
    end repeat
end uniqueDestination

on splitFilename(fileName)
    set AppleScript's text item delimiters to "."
    set parts to text items of fileName
    set AppleScript's text item delimiters to ""
    if (count of parts) > 1 then
        set extensionText to "." & (item -1 of parts)
        set baseText to (items 1 thru -2 of parts) as text
        return {baseText, extensionText}
    else
        return {fileName, ""}
    end if
end splitFilename

on listToText(theList)
    if (count of theList) is 0 then return "(none)"
    set AppleScript's text item delimiters to return
    set outputText to theList as text
    set AppleScript's text item delimiters to ""
    return outputText
end listToText

on sortTextList(theList)
    set sortedList to {}
    repeat with currentItem in theList
        set currentValue to contents of currentItem
        set insertPosition to (count of sortedList) + 1
        repeat with i from 1 to (count of sortedList)
            if currentValue comes before (item i of sortedList) then
                set insertPosition to i
                exit repeat
            end if
        end repeat
        if insertPosition > (count of sortedList) then
            set end of sortedList to currentValue
        else
            if insertPosition is 1 then
                set sortedList to {currentValue} & sortedList
            else
                set sortedList to (items 1 thru (insertPosition - 1) of sortedList) & {currentValue} & (items insertPosition thru -1 of sortedList)
            end if
        end if
    end repeat
    return sortedList
end sortTextList
