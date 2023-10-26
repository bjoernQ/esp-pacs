#[doc = "Register `DIG_ISO` reader"]
pub type R = crate::R<DIG_ISO_SPEC>;
#[doc = "Register `DIG_ISO` writer"]
pub type W = crate::W<DIG_ISO_SPEC>;
#[doc = "Field `FORCE_OFF` reader - "]
pub type FORCE_OFF_R = crate::BitReader;
#[doc = "Field `FORCE_OFF` writer - "]
pub type FORCE_OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FORCE_ON` reader - "]
pub type FORCE_ON_R = crate::BitReader;
#[doc = "Field `FORCE_ON` writer - "]
pub type FORCE_ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_PAD_AUTOHOLD` reader - Indicates the auto-hold status of the digital GPIOs."]
pub type DG_PAD_AUTOHOLD_R = crate::BitReader;
#[doc = "Field `CLR_DG_PAD_AUTOHOLD` writer - Se this bit to clear the auto-hold enabler for the digital GPIOs."]
pub type CLR_DG_PAD_AUTOHOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_PAD_AUTOHOLD_EN` reader - Se this bit to allow the digital GPIOs to enter the autohold status."]
pub type DG_PAD_AUTOHOLD_EN_R = crate::BitReader;
#[doc = "Field `DG_PAD_AUTOHOLD_EN` writer - Se this bit to allow the digital GPIOs to enter the autohold status."]
pub type DG_PAD_AUTOHOLD_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_PAD_FORCE_NOISO` reader - Set this bit to disable the force isolation to the digital GPIOs."]
pub type DG_PAD_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_NOISO` writer - Set this bit to disable the force isolation to the digital GPIOs."]
pub type DG_PAD_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_PAD_FORCE_ISO` reader - Set this bit to force isolate the digital GPIOs."]
pub type DG_PAD_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_ISO` writer - Set this bit to force isolate the digital GPIOs."]
pub type DG_PAD_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_PAD_FORCE_UNHOLD` reader - Set this bit the force unhold the digital GPIOs."]
pub type DG_PAD_FORCE_UNHOLD_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_UNHOLD` writer - Set this bit the force unhold the digital GPIOs."]
pub type DG_PAD_FORCE_UNHOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_PAD_FORCE_HOLD` reader - Set this bit the force hold the digital GPIOs."]
pub type DG_PAD_FORCE_HOLD_R = crate::BitReader;
#[doc = "Field `DG_PAD_FORCE_HOLD` writer - Set this bit the force hold the digital GPIOs."]
pub type DG_PAD_FORCE_HOLD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROM0_FORCE_ISO` reader - ROM force ISO"]
pub type ROM0_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `ROM0_FORCE_ISO` writer - ROM force ISO"]
pub type ROM0_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROM0_FORCE_NOISO` reader - ROM force no ISO"]
pub type ROM0_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `ROM0_FORCE_NOISO` writer - ROM force no ISO"]
pub type ROM0_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM0_FORCE_ISO` reader - internal SRAM 0 force ISO"]
pub type INTER_RAM0_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_FORCE_ISO` writer - internal SRAM 0 force ISO"]
pub type INTER_RAM0_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM0_FORCE_NOISO` reader - internal SRAM 0 force no ISO"]
pub type INTER_RAM0_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM0_FORCE_NOISO` writer - internal SRAM 0 force no ISO"]
pub type INTER_RAM0_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM1_FORCE_ISO` reader - internal SRAM 1 force ISO"]
pub type INTER_RAM1_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_FORCE_ISO` writer - internal SRAM 1 force ISO"]
pub type INTER_RAM1_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM1_FORCE_NOISO` reader - internal SRAM 1 force no ISO"]
pub type INTER_RAM1_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM1_FORCE_NOISO` writer - internal SRAM 1 force no ISO"]
pub type INTER_RAM1_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM2_FORCE_ISO` reader - internal SRAM 2 force ISO"]
pub type INTER_RAM2_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_FORCE_ISO` writer - internal SRAM 2 force ISO"]
pub type INTER_RAM2_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM2_FORCE_NOISO` reader - internal SRAM 2 force no ISO"]
pub type INTER_RAM2_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM2_FORCE_NOISO` writer - internal SRAM 2 force no ISO"]
pub type INTER_RAM2_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM3_FORCE_ISO` reader - internal SRAM 3 force ISO"]
pub type INTER_RAM3_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_FORCE_ISO` writer - internal SRAM 3 force ISO"]
pub type INTER_RAM3_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM3_FORCE_NOISO` reader - internal SRAM 3 force no ISO"]
pub type INTER_RAM3_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM3_FORCE_NOISO` writer - internal SRAM 3 force no ISO"]
pub type INTER_RAM3_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM4_FORCE_ISO` reader - internal SRAM 4 force ISO"]
pub type INTER_RAM4_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_FORCE_ISO` writer - internal SRAM 4 force ISO"]
pub type INTER_RAM4_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INTER_RAM4_FORCE_NOISO` reader - internal SRAM 4 force no ISO"]
pub type INTER_RAM4_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `INTER_RAM4_FORCE_NOISO` writer - internal SRAM 4 force no ISO"]
pub type INTER_RAM4_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIFI_FORCE_ISO` reader - Set this bit to force isolate the Wi-Fi circuits."]
pub type WIFI_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_ISO` writer - Set this bit to force isolate the Wi-Fi circuits."]
pub type WIFI_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WIFI_FORCE_NOISO` reader - Set this bit to disable the force isolation to the Wi-Fi circuits."]
pub type WIFI_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `WIFI_FORCE_NOISO` writer - Set this bit to disable the force isolation to the Wi-Fi circuits."]
pub type WIFI_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_WRAP_FORCE_ISO` reader - Set this bit to force isolate the digital system."]
pub type DG_WRAP_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_ISO` writer - Set this bit to force isolate the digital system."]
pub type DG_WRAP_FORCE_ISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DG_WRAP_FORCE_NOISO` reader - Set this bit to disable the force isolation to the digital system."]
pub type DG_WRAP_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `DG_WRAP_FORCE_NOISO` writer - Set this bit to disable the force isolation to the digital system."]
pub type DG_WRAP_FORCE_NOISO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn force_off(&self) -> FORCE_OFF_R {
        FORCE_OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn force_on(&self) -> FORCE_ON_R {
        FORCE_ON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates the auto-hold status of the digital GPIOs."]
    #[inline(always)]
    pub fn dg_pad_autohold(&self) -> DG_PAD_AUTOHOLD_R {
        DG_PAD_AUTOHOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Se this bit to allow the digital GPIOs to enter the autohold status."]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&self) -> DG_PAD_AUTOHOLD_EN_R {
        DG_PAD_AUTOHOLD_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to disable the force isolation to the digital GPIOs."]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&self) -> DG_PAD_FORCE_NOISO_R {
        DG_PAD_FORCE_NOISO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to force isolate the digital GPIOs."]
    #[inline(always)]
    pub fn dg_pad_force_iso(&self) -> DG_PAD_FORCE_ISO_R {
        DG_PAD_FORCE_ISO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit the force unhold the digital GPIOs."]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&self) -> DG_PAD_FORCE_UNHOLD_R {
        DG_PAD_FORCE_UNHOLD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit the force hold the digital GPIOs."]
    #[inline(always)]
    pub fn dg_pad_force_hold(&self) -> DG_PAD_FORCE_HOLD_R {
        DG_PAD_FORCE_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ROM force ISO"]
    #[inline(always)]
    pub fn rom0_force_iso(&self) -> ROM0_FORCE_ISO_R {
        ROM0_FORCE_ISO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ROM force no ISO"]
    #[inline(always)]
    pub fn rom0_force_noiso(&self) -> ROM0_FORCE_NOISO_R {
        ROM0_FORCE_NOISO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - internal SRAM 0 force ISO"]
    #[inline(always)]
    pub fn inter_ram0_force_iso(&self) -> INTER_RAM0_FORCE_ISO_R {
        INTER_RAM0_FORCE_ISO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - internal SRAM 0 force no ISO"]
    #[inline(always)]
    pub fn inter_ram0_force_noiso(&self) -> INTER_RAM0_FORCE_NOISO_R {
        INTER_RAM0_FORCE_NOISO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - internal SRAM 1 force ISO"]
    #[inline(always)]
    pub fn inter_ram1_force_iso(&self) -> INTER_RAM1_FORCE_ISO_R {
        INTER_RAM1_FORCE_ISO_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - internal SRAM 1 force no ISO"]
    #[inline(always)]
    pub fn inter_ram1_force_noiso(&self) -> INTER_RAM1_FORCE_NOISO_R {
        INTER_RAM1_FORCE_NOISO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - internal SRAM 2 force ISO"]
    #[inline(always)]
    pub fn inter_ram2_force_iso(&self) -> INTER_RAM2_FORCE_ISO_R {
        INTER_RAM2_FORCE_ISO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - internal SRAM 2 force no ISO"]
    #[inline(always)]
    pub fn inter_ram2_force_noiso(&self) -> INTER_RAM2_FORCE_NOISO_R {
        INTER_RAM2_FORCE_NOISO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - internal SRAM 3 force ISO"]
    #[inline(always)]
    pub fn inter_ram3_force_iso(&self) -> INTER_RAM3_FORCE_ISO_R {
        INTER_RAM3_FORCE_ISO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - internal SRAM 3 force no ISO"]
    #[inline(always)]
    pub fn inter_ram3_force_noiso(&self) -> INTER_RAM3_FORCE_NOISO_R {
        INTER_RAM3_FORCE_NOISO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - internal SRAM 4 force ISO"]
    #[inline(always)]
    pub fn inter_ram4_force_iso(&self) -> INTER_RAM4_FORCE_ISO_R {
        INTER_RAM4_FORCE_ISO_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - internal SRAM 4 force no ISO"]
    #[inline(always)]
    pub fn inter_ram4_force_noiso(&self) -> INTER_RAM4_FORCE_NOISO_R {
        INTER_RAM4_FORCE_NOISO_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to force isolate the Wi-Fi circuits."]
    #[inline(always)]
    pub fn wifi_force_iso(&self) -> WIFI_FORCE_ISO_R {
        WIFI_FORCE_ISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to disable the force isolation to the Wi-Fi circuits."]
    #[inline(always)]
    pub fn wifi_force_noiso(&self) -> WIFI_FORCE_NOISO_R {
        WIFI_FORCE_NOISO_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to force isolate the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&self) -> DG_WRAP_FORCE_ISO_R {
        DG_WRAP_FORCE_ISO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to disable the force isolation to the digital system."]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&self) -> DG_WRAP_FORCE_NOISO_R {
        DG_WRAP_FORCE_NOISO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_ISO")
            .field("force_off", &format_args!("{}", self.force_off().bit()))
            .field("force_on", &format_args!("{}", self.force_on().bit()))
            .field(
                "dg_pad_autohold",
                &format_args!("{}", self.dg_pad_autohold().bit()),
            )
            .field(
                "dg_pad_autohold_en",
                &format_args!("{}", self.dg_pad_autohold_en().bit()),
            )
            .field(
                "dg_pad_force_noiso",
                &format_args!("{}", self.dg_pad_force_noiso().bit()),
            )
            .field(
                "dg_pad_force_iso",
                &format_args!("{}", self.dg_pad_force_iso().bit()),
            )
            .field(
                "dg_pad_force_unhold",
                &format_args!("{}", self.dg_pad_force_unhold().bit()),
            )
            .field(
                "dg_pad_force_hold",
                &format_args!("{}", self.dg_pad_force_hold().bit()),
            )
            .field(
                "rom0_force_iso",
                &format_args!("{}", self.rom0_force_iso().bit()),
            )
            .field(
                "rom0_force_noiso",
                &format_args!("{}", self.rom0_force_noiso().bit()),
            )
            .field(
                "inter_ram0_force_iso",
                &format_args!("{}", self.inter_ram0_force_iso().bit()),
            )
            .field(
                "inter_ram0_force_noiso",
                &format_args!("{}", self.inter_ram0_force_noiso().bit()),
            )
            .field(
                "inter_ram1_force_iso",
                &format_args!("{}", self.inter_ram1_force_iso().bit()),
            )
            .field(
                "inter_ram1_force_noiso",
                &format_args!("{}", self.inter_ram1_force_noiso().bit()),
            )
            .field(
                "inter_ram2_force_iso",
                &format_args!("{}", self.inter_ram2_force_iso().bit()),
            )
            .field(
                "inter_ram2_force_noiso",
                &format_args!("{}", self.inter_ram2_force_noiso().bit()),
            )
            .field(
                "inter_ram3_force_iso",
                &format_args!("{}", self.inter_ram3_force_iso().bit()),
            )
            .field(
                "inter_ram3_force_noiso",
                &format_args!("{}", self.inter_ram3_force_noiso().bit()),
            )
            .field(
                "inter_ram4_force_iso",
                &format_args!("{}", self.inter_ram4_force_iso().bit()),
            )
            .field(
                "inter_ram4_force_noiso",
                &format_args!("{}", self.inter_ram4_force_noiso().bit()),
            )
            .field(
                "wifi_force_iso",
                &format_args!("{}", self.wifi_force_iso().bit()),
            )
            .field(
                "wifi_force_noiso",
                &format_args!("{}", self.wifi_force_noiso().bit()),
            )
            .field(
                "dg_wrap_force_iso",
                &format_args!("{}", self.dg_wrap_force_iso().bit()),
            )
            .field(
                "dg_wrap_force_noiso",
                &format_args!("{}", self.dg_wrap_force_noiso().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIG_ISO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn force_off(&mut self) -> FORCE_OFF_W<DIG_ISO_SPEC, 7> {
        FORCE_OFF_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn force_on(&mut self) -> FORCE_ON_W<DIG_ISO_SPEC, 8> {
        FORCE_ON_W::new(self)
    }
    #[doc = "Bit 10 - Se this bit to clear the auto-hold enabler for the digital GPIOs."]
    #[inline(always)]
    #[must_use]
    pub fn clr_dg_pad_autohold(&mut self) -> CLR_DG_PAD_AUTOHOLD_W<DIG_ISO_SPEC, 10> {
        CLR_DG_PAD_AUTOHOLD_W::new(self)
    }
    #[doc = "Bit 11 - Se this bit to allow the digital GPIOs to enter the autohold status."]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_autohold_en(&mut self) -> DG_PAD_AUTOHOLD_EN_W<DIG_ISO_SPEC, 11> {
        DG_PAD_AUTOHOLD_EN_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to disable the force isolation to the digital GPIOs."]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_noiso(&mut self) -> DG_PAD_FORCE_NOISO_W<DIG_ISO_SPEC, 12> {
        DG_PAD_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to force isolate the digital GPIOs."]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_iso(&mut self) -> DG_PAD_FORCE_ISO_W<DIG_ISO_SPEC, 13> {
        DG_PAD_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit the force unhold the digital GPIOs."]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_unhold(&mut self) -> DG_PAD_FORCE_UNHOLD_W<DIG_ISO_SPEC, 14> {
        DG_PAD_FORCE_UNHOLD_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit the force hold the digital GPIOs."]
    #[inline(always)]
    #[must_use]
    pub fn dg_pad_force_hold(&mut self) -> DG_PAD_FORCE_HOLD_W<DIG_ISO_SPEC, 15> {
        DG_PAD_FORCE_HOLD_W::new(self)
    }
    #[doc = "Bit 16 - ROM force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_force_iso(&mut self) -> ROM0_FORCE_ISO_W<DIG_ISO_SPEC, 16> {
        ROM0_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 17 - ROM force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn rom0_force_noiso(&mut self) -> ROM0_FORCE_NOISO_W<DIG_ISO_SPEC, 17> {
        ROM0_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 18 - internal SRAM 0 force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_force_iso(&mut self) -> INTER_RAM0_FORCE_ISO_W<DIG_ISO_SPEC, 18> {
        INTER_RAM0_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 19 - internal SRAM 0 force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram0_force_noiso(&mut self) -> INTER_RAM0_FORCE_NOISO_W<DIG_ISO_SPEC, 19> {
        INTER_RAM0_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 20 - internal SRAM 1 force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_force_iso(&mut self) -> INTER_RAM1_FORCE_ISO_W<DIG_ISO_SPEC, 20> {
        INTER_RAM1_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 21 - internal SRAM 1 force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram1_force_noiso(&mut self) -> INTER_RAM1_FORCE_NOISO_W<DIG_ISO_SPEC, 21> {
        INTER_RAM1_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 22 - internal SRAM 2 force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_force_iso(&mut self) -> INTER_RAM2_FORCE_ISO_W<DIG_ISO_SPEC, 22> {
        INTER_RAM2_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 23 - internal SRAM 2 force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram2_force_noiso(&mut self) -> INTER_RAM2_FORCE_NOISO_W<DIG_ISO_SPEC, 23> {
        INTER_RAM2_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 24 - internal SRAM 3 force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_force_iso(&mut self) -> INTER_RAM3_FORCE_ISO_W<DIG_ISO_SPEC, 24> {
        INTER_RAM3_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 25 - internal SRAM 3 force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram3_force_noiso(&mut self) -> INTER_RAM3_FORCE_NOISO_W<DIG_ISO_SPEC, 25> {
        INTER_RAM3_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 26 - internal SRAM 4 force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_force_iso(&mut self) -> INTER_RAM4_FORCE_ISO_W<DIG_ISO_SPEC, 26> {
        INTER_RAM4_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 27 - internal SRAM 4 force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn inter_ram4_force_noiso(&mut self) -> INTER_RAM4_FORCE_NOISO_W<DIG_ISO_SPEC, 27> {
        INTER_RAM4_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to force isolate the Wi-Fi circuits."]
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_iso(&mut self) -> WIFI_FORCE_ISO_W<DIG_ISO_SPEC, 28> {
        WIFI_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to disable the force isolation to the Wi-Fi circuits."]
    #[inline(always)]
    #[must_use]
    pub fn wifi_force_noiso(&mut self) -> WIFI_FORCE_NOISO_W<DIG_ISO_SPEC, 29> {
        WIFI_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to force isolate the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_iso(&mut self) -> DG_WRAP_FORCE_ISO_W<DIG_ISO_SPEC, 30> {
        DG_WRAP_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to disable the force isolation to the digital system."]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_force_noiso(&mut self) -> DG_WRAP_FORCE_NOISO_W<DIG_ISO_SPEC, 31> {
        DG_WRAP_FORCE_NOISO_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Digital system ISO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_iso::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_iso::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIG_ISO_SPEC;
impl crate::RegisterSpec for DIG_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_iso::R`](R) reader structure"]
impl crate::Readable for DIG_ISO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dig_iso::W`](W) writer structure"]
impl crate::Writable for DIG_ISO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIG_ISO to value 0xaaaa_5000"]
impl crate::Resettable for DIG_ISO_SPEC {
    const RESET_VALUE: Self::Ux = 0xaaaa_5000;
}
