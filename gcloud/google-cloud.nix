{ pkgs ? import <nixpkgs> {} }:

pkgs.google-cloud-sdk.overrideAttrs(old: rec {
  version = "304.0.0";
  pythonEnv = pkgs.python.withPackages (p:
    with p;
    [ cffi cryptography pyopenssl crcmod ]);
  src = pkgs.fetchurl {
    url    = "https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-304.0.0-linux-x86_64.tar.gz";
    sha256 = "9cc7ec94855c3cab99211879ab8774a5a16b8de5322be5dd055e8da6d80495d3";
  };
  installPhase = ''
    mkdir -p $out/google-cloud-sdk
    cp -R * .install $out/google-cloud-sdk/

    mkdir -p $out/google-cloud-sdk/lib/surface/{alpha,beta}
    cp ${./alpha__init__.py} $out/google-cloud-sdk/lib/surface/alpha/__init__.py
    cp ${./beta__init__.py} $out/google-cloud-sdk/lib/surface/beta/__init__.py

    # create wrappers with correct env
    for program in gcloud bq gsutil git-credential-gcloud.sh docker-credential-gcloud; do
        programPath="$out/google-cloud-sdk/bin/$program"
        binaryPath="$out/bin/$program"
        wrapProgram "$programPath" \
            --set CLOUDSDK_PYTHON "${pythonEnv}/bin/python" \
            --prefix PYTHONPATH : "${pythonEnv}/${pkgs.python.sitePackages}" \
            --prefix PATH : "${pkgs.openssl.bin}/bin"

        mkdir -p $out/bin
        ln -s $programPath $binaryPath
    done

    # disable component updater and update check
    substituteInPlace $out/google-cloud-sdk/lib/googlecloudsdk/core/config.json \
      --replace "\"disable_updater\": false" "\"disable_updater\": true"
    echo "
    [component_manager]
    disable_update_check = true" >> $out/google-cloud-sdk/properties

    # setup bash completion
    mkdir -p $out/etc/bash_completion.d
    mv $out/google-cloud-sdk/completion.bash.inc $out/etc/bash_completion.d/gcloud.inc

    # This directory contains compiled mac binaries. We used crcmod from
    # nixpkgs instead.
    rm -r $out/google-cloud-sdk/platform/gsutil/third_party/crcmod \
          $out/google-cloud-sdk/platform/gsutil/third_party/crcmod_osx

    # remove tests and test data
    find $out -name tests -type d -exec rm -rf '{}' +
    rm $out/google-cloud-sdk/platform/gsutil/gslib/commands/test.py

    # compact all the JSON
    find $out -name \*.json | while read path; do
      jq -c . $path > $path.min
      mv $path.min $path
    done
  '';
})
