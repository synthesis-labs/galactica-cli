# Download galactica exe into your user/.galactica folder and add it to path
# You might need to bypass the restriction to run scripts:
# PowerShell -ExecutionPolicy Bypass

$GalacticaFolderPath = "$env:USERPROFILE\.galactica"
$DownloadUrl = "https://github.com/synthesis-labs/galactica-cli/releases/download/beta-rc.1/galactica-x86_64-pc-windows-gnu-0.1.0-build.17.014b5bf.zip"

# Creating a new directory called .galactica under user's home directory
if(!(Test-Path $GalacticaFolderPath)) {
    New-Item -ItemType Directory -Path $GalacticaFolderPath
}

# Downloading the zip file
Invoke-WebRequest -Uri $DownloadUrl -OutFile "$GalacticaFolderPath\galactica.zip"

# Unzipping the downloaded zip file into .galactica directory
Expand-Archive -Path "$GalacticaFolderPath\galactica.zip" -DestinationPath $GalacticaFolderPath -Force

# Deleting the zip file from the .galactica directory
Remove-Item "$GalacticaFolderPath\galactica.zip"

# Adding the .galactica directory to the Windows PATH environment variable
$ExistingPath = [Environment]::GetEnvironmentVariable("Path","Machine")
if (!($ExistingPath.Split(";") -contains $GalacticaFolderPath)) {
  $NewPath = "$ExistingPath;$GalacticaFolderPath"
  [Environment]::SetEnvironmentVariable("Path", $NewPath, "Machine")
}