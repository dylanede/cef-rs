cargo build --release
xcopy /S /Y %CEF_DIST_ROOT%\Release\* target\release
xcopy /S /Y %CEF_DIST_ROOT%\Resources\* target\release

