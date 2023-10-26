#[doc = "Register `MULT_CONF` reader"]
pub type R = crate::R<MULT_CONF_SPEC>;
#[doc = "Register `MULT_CONF` writer"]
pub type W = crate::W<MULT_CONF_SPEC>;
#[doc = "Field `START` reader - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESET` writer - Write 1 to reset ECC Accelerator."]
pub type RESET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KEY_LENGTH` reader - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
pub type KEY_LENGTH_R = crate::BitReader;
#[doc = "Field `KEY_LENGTH` writer - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
pub type KEY_LENGTH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SECURITY_MODE` reader - Reserved"]
pub type SECURITY_MODE_R = crate::BitReader;
#[doc = "Field `SECURITY_MODE` writer - Reserved"]
pub type SECURITY_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLK_EN` reader - Write 1 to force on register clock gate."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Write 1 to force on register clock gate."]
pub type CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WORK_MODE` reader - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
pub type WORK_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `VERIFICATION_RESULT` reader - The verification result bit of ECC Accelerator, only valid when calculation is done."]
pub type VERIFICATION_RESULT_R = crate::BitReader;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` reader - ECC memory clock gate force on register"]
pub type MEM_CLOCK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` writer - ECC memory clock gate force on register"]
pub type MEM_CLOCK_GATE_FORCE_ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
    #[inline(always)]
    pub fn key_length(&self) -> KEY_LENGTH_R {
        KEY_LENGTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn security_mode(&self) -> SECURITY_MODE_R {
        SECURITY_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to force on register clock gate."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - The verification result bit of ECC Accelerator, only valid when calculation is done."]
    #[inline(always)]
    pub fn verification_result(&self) -> VERIFICATION_RESULT_R {
        VERIFICATION_RESULT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC memory clock gate force on register"]
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&self) -> MEM_CLOCK_GATE_FORCE_ON_R {
        MEM_CLOCK_GATE_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_CONF")
            .field("start", &format_args!("{}", self.start().bit()))
            .field("key_length", &format_args!("{}", self.key_length().bit()))
            .field(
                "security_mode",
                &format_args!("{}", self.security_mode().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field("work_mode", &format_args!("{}", self.work_mode().bits()))
            .field(
                "verification_result",
                &format_args!("{}", self.verification_result().bit()),
            )
            .field(
                "mem_clock_gate_force_on",
                &format_args!("{}", self.mem_clock_gate_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<MULT_CONF_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 to reset ECC Accelerator."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<MULT_CONF_SPEC, 1> {
        RESET_W::new(self)
    }
    #[doc = "Bit 2 - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
    #[inline(always)]
    #[must_use]
    pub fn key_length(&mut self) -> KEY_LENGTH_W<MULT_CONF_SPEC, 2> {
        KEY_LENGTH_W::new(self)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn security_mode(&mut self) -> SECURITY_MODE_W<MULT_CONF_SPEC, 3> {
        SECURITY_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Write 1 to force on register clock gate."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<MULT_CONF_SPEC, 4> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 5:7 - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
    #[inline(always)]
    #[must_use]
    pub fn work_mode(&mut self) -> WORK_MODE_W<MULT_CONF_SPEC, 5> {
        WORK_MODE_W::new(self)
    }
    #[doc = "Bit 31 - ECC memory clock gate force on register"]
    #[inline(always)]
    #[must_use]
    pub fn mem_clock_gate_force_on(&mut self) -> MEM_CLOCK_GATE_FORCE_ON_W<MULT_CONF_SPEC, 31> {
        MEM_CLOCK_GATE_FORCE_ON_W::new(self)
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
#[doc = "ECC configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mult_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_CONF_SPEC;
impl crate::RegisterSpec for MULT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_conf::R`](R) reader structure"]
impl crate::Readable for MULT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_conf::W`](W) writer structure"]
impl crate::Writable for MULT_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MULT_CONF to value 0x8000_0000"]
impl crate::Resettable for MULT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
