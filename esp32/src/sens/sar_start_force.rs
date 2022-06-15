#[doc = "Register `SAR_START_FORCE` reader"]
pub struct R(crate::R<SAR_START_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_START_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_START_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_START_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_START_FORCE` writer"]
pub struct W(crate::W<SAR_START_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_START_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SAR_START_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_START_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR1_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR1_BIT_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR1_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR1_BIT_WIDTH_W<'a> = crate::FieldWriter<'a, u32, SAR_START_FORCE_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SAR2_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR2_BIT_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR2_BIT_WIDTH_W<'a> = crate::FieldWriter<'a, u32, SAR_START_FORCE_SPEC, u8, u8, 2, 2>;
#[doc = "Field `SAR2_EN_TEST` reader - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub type SAR2_EN_TEST_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_EN_TEST` writer - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub type SAR2_EN_TEST_W<'a> = crate::BitWriter<'a, u32, SAR_START_FORCE_SPEC, bool, 4>;
#[doc = "Field `SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_W<'a> = crate::FieldWriter<'a, u32, SAR_START_FORCE_SPEC, u8, u8, 3, 5>;
#[doc = "Field `ULP_CP_FORCE_START_TOP` reader - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub type ULP_CP_FORCE_START_TOP_R = crate::BitReader<bool>;
#[doc = "Field `ULP_CP_FORCE_START_TOP` writer - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub type ULP_CP_FORCE_START_TOP_W<'a> = crate::BitWriter<'a, u32, SAR_START_FORCE_SPEC, bool, 8>;
#[doc = "Field `ULP_CP_START_TOP` reader - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub type ULP_CP_START_TOP_R = crate::BitReader<bool>;
#[doc = "Field `ULP_CP_START_TOP` writer - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub type ULP_CP_START_TOP_W<'a> = crate::BitWriter<'a, u32, SAR_START_FORCE_SPEC, bool, 9>;
#[doc = "Field `SARCLK_EN` reader - "]
pub type SARCLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `SARCLK_EN` writer - "]
pub type SARCLK_EN_W<'a> = crate::BitWriter<'a, u32, SAR_START_FORCE_SPEC, bool, 10>;
#[doc = "Field `PC_INIT` reader - initialized PC for ULP-coprocessor"]
pub type PC_INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PC_INIT` writer - initialized PC for ULP-coprocessor"]
pub type PC_INIT_W<'a> = crate::FieldWriter<'a, u32, SAR_START_FORCE_SPEC, u16, u16, 11, 11>;
#[doc = "Field `SAR2_STOP` reader - stop SAR ADC2 conversion"]
pub type SAR2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_STOP` writer - stop SAR ADC2 conversion"]
pub type SAR2_STOP_W<'a> = crate::BitWriter<'a, u32, SAR_START_FORCE_SPEC, bool, 22>;
#[doc = "Field `SAR1_STOP` reader - stop SAR ADC1 conversion"]
pub type SAR1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SAR1_STOP` writer - stop SAR ADC1 conversion"]
pub type SAR1_STOP_W<'a> = crate::BitWriter<'a, u32, SAR_START_FORCE_SPEC, bool, 23>;
#[doc = "Field `SAR2_PWDET_EN` reader - N/A"]
pub type SAR2_PWDET_EN_R = crate::BitReader<bool>;
#[doc = "Field `SAR2_PWDET_EN` writer - N/A"]
pub type SAR2_PWDET_EN_W<'a> = crate::BitWriter<'a, u32, SAR_START_FORCE_SPEC, bool, 24>;
impl R {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar1_bit_width(&self) -> SAR1_BIT_WIDTH_R {
        SAR1_BIT_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar2_bit_width(&self) -> SAR2_BIT_WIDTH_R {
        SAR2_BIT_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sar2_en_test(&self) -> SAR2_EN_TEST_R {
        SAR2_EN_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&self) -> ULP_CP_FORCE_START_TOP_R {
        ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&self) -> ULP_CP_START_TOP_R {
        ULP_CP_START_TOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sarclk_en(&self) -> SARCLK_EN_R {
        SARCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn pc_init(&self) -> PC_INIT_R {
        PC_INIT_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sar2_stop(&self) -> SAR2_STOP_R {
        SAR2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sar1_stop(&self) -> SAR1_STOP_R {
        SAR1_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sar2_pwdet_en(&self) -> SAR2_PWDET_EN_R {
        SAR2_PWDET_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar1_bit_width(&mut self) -> SAR1_BIT_WIDTH_W {
        SAR1_BIT_WIDTH_W::new(self)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar2_bit_width(&mut self) -> SAR2_BIT_WIDTH_W {
        SAR2_BIT_WIDTH_W::new(self)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sar2_en_test(&mut self) -> SAR2_EN_TEST_W {
        SAR2_EN_TEST_W::new(self)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W {
        SAR2_PWDET_CCT_W::new(self)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&mut self) -> ULP_CP_FORCE_START_TOP_W {
        ULP_CP_FORCE_START_TOP_W::new(self)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&mut self) -> ULP_CP_START_TOP_W {
        ULP_CP_START_TOP_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sarclk_en(&mut self) -> SARCLK_EN_W {
        SARCLK_EN_W::new(self)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn pc_init(&mut self) -> PC_INIT_W {
        PC_INIT_W::new(self)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sar2_stop(&mut self) -> SAR2_STOP_W {
        SAR2_STOP_W::new(self)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sar1_stop(&mut self) -> SAR1_STOP_W {
        SAR1_STOP_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sar2_pwdet_en(&mut self) -> SAR2_PWDET_EN_W {
        SAR2_PWDET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_start_force](index.html) module"]
pub struct SAR_START_FORCE_SPEC;
impl crate::RegisterSpec for SAR_START_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_start_force::R](R) reader structure"]
impl crate::Readable for SAR_START_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_start_force::W](W) writer structure"]
impl crate::Writable for SAR_START_FORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAR_START_FORCE to value 0x0f"]
impl crate::Resettable for SAR_START_FORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
