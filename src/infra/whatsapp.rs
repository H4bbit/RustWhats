use qrcode::QrCode;

/// Renderiza QR code no terminal
pub fn print_qr(payload: &str) {
    if let Ok(qr) = QrCode::new(payload.as_bytes()) {
        let rendered = qr
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();

        println!("{}", rendered);
    }
}
